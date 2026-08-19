//! Read-only provider for a published iCalendar feed.
//!
//! Published calendar URLs are bearer credentials. Callers persist the URL in
//! the encrypted credential column and keep only [`PRIVATE_URL_SENTINEL`] in
//! the ordinary source URL column so dashboards, exports, and logs cannot
//! disclose it.

use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;

use super::{CalendarProvider, DeltaResult, RawEvent, RemoteCalendar};

pub const PRIVATE_URL_SENTINEL: &str = "https://published-calendar.invalid/private.ics";
pub(crate) const CALENDAR_ID: &str = "published-ics";

pub struct PublishedIcsProvider {
    feed_url: String,
}

impl PublishedIcsProvider {
    pub fn new(feed_url: &str) -> Self {
        Self {
            feed_url: feed_url.to_string(),
        }
    }

    async fn fetch(&self) -> Result<String> {
        crate::resources::fetch_feed(&self.feed_url).await
    }

    async fn fetch_future(&self) -> Result<String> {
        Ok(filter_future_vevents(&self.fetch().await?, Utc::now()))
    }
}

fn event_datetime_utc(vevent: &str, field: &str) -> Option<DateTime<Utc>> {
    let value = crate::utils::extract_vevent_field(vevent, field)?;
    let naive = crate::utils::parse_ical_datetime(&value)?;
    let tzid = crate::utils::extract_vevent_tzid(vevent, field);
    match tzid.and_then(|value| value.parse::<Tz>().ok()) {
        Some(tz) => tz
            .from_local_datetime(&naive)
            .earliest()
            .map(|value| value.with_timezone(&Utc)),
        None => Some(Utc.from_utc_datetime(&naive)),
    }
}

fn local_now_for_event(vevent: &str, now: DateTime<Utc>) -> NaiveDateTime {
    crate::utils::extract_vevent_tzid(vevent, "DTSTART")
        .and_then(|value| value.parse::<Tz>().ok())
        .map(|tz| now.with_timezone(&tz).naive_local())
        .unwrap_or_else(|| now.naive_utc())
}

fn rrule_until_utc(vevent: &str, rrule: &str) -> Option<DateTime<Utc>> {
    let raw = rrule
        .split(';')
        .find_map(|part| part.strip_prefix("UNTIL="))?;
    let mut naive = crate::utils::parse_ical_datetime(raw)?;
    if raw.trim_end_matches('Z').len() == 8 {
        naive = naive.date().and_hms_opt(23, 59, 59)?;
    }
    if raw.ends_with('Z') {
        return Some(Utc.from_utc_datetime(&naive));
    }
    let tzid = crate::utils::extract_vevent_tzid(vevent, "DTSTART");
    match tzid.and_then(|value| value.parse::<Tz>().ok()) {
        Some(tz) => tz
            .from_local_datetime(&naive)
            .earliest()
            .map(|value| value.with_timezone(&Utc)),
        None => Some(Utc.from_utc_datetime(&naive)),
    }
}

fn recurring_event_has_future_occurrence(
    vevent: &str,
    rrule: &str,
    now: DateTime<Utc>,
) -> bool {
    // Cal.rs currently expands these frequencies. Keep unknown frequencies so
    // we fail open for availability rather than silently dropping a series.
    let supported = ["FREQ=DAILY", "FREQ=WEEKLY", "FREQ=MONTHLY"]
        .iter()
        .any(|freq| rrule.contains(freq));
    if !supported {
        return true;
    }

    if let Some(until) = rrule_until_utc(vevent, rrule) {
        return until >= now;
    }
    // An unbounded rule necessarily remains relevant. COUNT-bounded rules
    // need expansion to determine whether their final instance has passed.
    if !rrule.split(';').any(|part| part.starts_with("COUNT=")) {
        return true;
    }

    let Some(start) = crate::utils::extract_vevent_field(vevent, "DTSTART")
        .and_then(|value| crate::utils::parse_ical_datetime(&value))
    else {
        return false;
    };
    let end = crate::utils::extract_vevent_field(vevent, "DTEND")
        .and_then(|value| crate::utils::parse_ical_datetime(&value))
        .unwrap_or(start);
    let window_start = local_now_for_event(vevent, now);
    let window_end = window_start + Duration::days(730);
    !crate::rrule::expand_rrule(
        start,
        end,
        rrule,
        &crate::rrule::extract_exdates(vevent),
        window_start,
        window_end,
    )
    .is_empty()
}

/// Published feeds can contain years of history. Retain only entries that can
/// affect availability now or in the product's two-year scheduling horizon.
fn filter_future_vevents(ical: &str, now: DateTime<Utc>) -> String {
    crate::utils::split_vevents(ical)
        .into_iter()
        .filter(|vevent| {
            if let Some(rrule) = crate::utils::extract_vevent_field(vevent, "RRULE") {
                recurring_event_has_future_occurrence(vevent, &rrule, now)
            } else {
                event_datetime_utc(vevent, "DTEND")
                    .or_else(|| event_datetime_utc(vevent, "DTSTART"))
                    .is_some_and(|end| end > now)
            }
        })
        .collect::<Vec<_>>()
        .join("\r\n")
}

/// Accept a direct HTTPS feed, a `webcal://` subscription URL, or Outlook's
/// `addsubscription` wrapper and return the canonical HTTPS feed URL.
pub fn normalize_subscription_url(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("Published calendar URL is required");
    }

    let candidate = if trimmed.starts_with("https://outlook.live.com/") {
        let outer = reqwest::Url::parse(trimmed)?;
        let is_add_subscription = outer
            .query_pairs()
            .any(|(key, value)| key == "rru" && value == "addsubscription");
        if !is_add_subscription {
            bail!("Outlook subscription link is missing rru=addsubscription");
        }
        outer
            .query_pairs()
            .find_map(|(key, value)| (key == "url").then(|| value.into_owned()))
            .ok_or_else(|| {
                anyhow::anyhow!("Outlook subscription link is missing its calendar URL")
            })?
    } else {
        trimmed.to_string()
    };

    let https_candidate = if let Some(rest) = candidate.strip_prefix("webcal://") {
        format!("https://{}", rest)
    } else {
        candidate
    };
    let mut parsed = reqwest::Url::parse(&https_candidate)?;
    if parsed.scheme() != "https" {
        bail!("Published calendar URLs must use HTTPS or webcal");
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        bail!("Published calendar URL must not contain embedded credentials");
    }
    parsed.set_fragment(None);
    Ok(parsed.to_string())
}

/// Normalize and apply the same DNS/SSRF checks used for other remote
/// calendar sources.
pub fn validate_subscription_url(input: &str) -> Result<String> {
    let normalized = normalize_subscription_url(input)?;
    crate::caldav::validate_caldav_url(&normalized)?;
    Ok(normalized)
}

#[async_trait]
impl CalendarProvider for PublishedIcsProvider {
    async fn check_connection(&self) -> Result<bool> {
        self.fetch().await?;
        Ok(true)
    }

    async fn list_calendars(&self) -> Result<Vec<RemoteCalendar>> {
        Ok(vec![RemoteCalendar {
            id: CALENDAR_ID.to_string(),
            display_name: Some("Published calendar".to_string()),
            color: None,
            change_marker: None,
            sync_state: None,
        }])
    }

    async fn fetch_events(&self, calendar_id: &str) -> Result<Vec<RawEvent>> {
        if calendar_id != CALENDAR_ID {
            bail!("Unknown published calendar");
        }
        let ical = self.fetch_future().await?;
        if ical.is_empty() {
            return Ok(Vec::new());
        }
        Ok(vec![RawEvent {
            remote_id: CALENDAR_ID.to_string(),
            ical,
        }])
    }

    async fn fetch_events_since(
        &self,
        calendar_id: &str,
        _since_utc: &str,
    ) -> Result<Vec<RawEvent>> {
        self.fetch_events(calendar_id).await
    }

    async fn sync_delta(
        &self,
        _calendar_id: &str,
        _sync_state: Option<&str>,
    ) -> Result<DeltaResult> {
        Ok(DeltaResult::default())
    }

    async fn put_event(&self, _calendar_id: &str, _uid: &str, _ics: &str) -> Result<()> {
        bail!("Published calendar feeds are read-only")
    }

    async fn delete_event(&self, _calendar_id: &str, _uid: &str) -> Result<()> {
        bail!("Published calendar feeds are read-only")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_direct_https_url() {
        assert_eq!(
            normalize_subscription_url("https://example.com/calendar.ics").unwrap(),
            "https://example.com/calendar.ics"
        );
    }

    #[test]
    fn normalizes_webcal_url() {
        assert_eq!(
            normalize_subscription_url("webcal://example.com/calendar.ics").unwrap(),
            "https://example.com/calendar.ics"
        );
    }

    #[test]
    fn extracts_outlook_subscription_wrapper() {
        let wrapped = "https://outlook.live.com/?rru=addsubscription&url=webcal%3A%2F%2Fexample.com%2Fbusy.ics&name=Busy";
        assert_eq!(
            normalize_subscription_url(wrapped).unwrap(),
            "https://example.com/busy.ics"
        );
    }

    #[test]
    fn rejects_insecure_urls() {
        assert!(normalize_subscription_url("http://example.com/calendar.ics").is_err());
    }

    #[test]
    fn keeps_only_future_entries_and_live_recurring_series() {
        let now = Utc.with_ymd_and_hms(2026, 8, 19, 12, 0, 0).unwrap();
        let body = "BEGIN:VCALENDAR\r\n\
BEGIN:VEVENT\r\nUID:past\r\nDTSTART:20260818T100000Z\r\nDTEND:20260818T110000Z\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:future\r\nDTSTART;TZID=Eastern Standard Time:20260819T100000\r\nDTEND;TZID=Eastern Standard Time:20260819T110000\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:expired-series\r\nDTSTART:20260801T100000Z\r\nDTEND:20260801T110000Z\r\nRRULE:FREQ=DAILY;UNTIL=20260818T100000Z\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:live-series\r\nDTSTART:20260801T100000Z\r\nDTEND:20260801T110000Z\r\nRRULE:FREQ=WEEKLY;UNTIL=20260930T100000Z\r\nEND:VEVENT\r\n\
END:VCALENDAR\r\n";
        let filtered = filter_future_vevents(body, now);
        assert!(!filtered.contains("UID:past"));
        assert!(filtered.contains("UID:future"));
        assert!(!filtered.contains("UID:expired-series"));
        assert!(filtered.contains("UID:live-series"));
        assert!(!filtered.contains("BEGIN:VCALENDAR"));
    }
}
