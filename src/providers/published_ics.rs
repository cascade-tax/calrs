//! Read-only provider for a published iCalendar feed.
//!
//! Published calendar URLs are bearer credentials. Callers persist the URL in
//! the encrypted credential column and keep only [`PRIVATE_URL_SENTINEL`] in
//! the ordinary source URL column so dashboards, exports, and logs cannot
//! disclose it.

use anyhow::{bail, Result};
use async_trait::async_trait;

use super::{CalendarProvider, DeltaResult, RawEvent, RemoteCalendar};

pub const PRIVATE_URL_SENTINEL: &str = "https://published-calendar.invalid/private.ics";
const CALENDAR_ID: &str = "published-ics";

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
            .ok_or_else(|| anyhow::anyhow!("Outlook subscription link is missing its calendar URL"))?
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
        Ok(vec![RawEvent {
            remote_id: CALENDAR_ID.to_string(),
            ical: self.fetch().await?,
        }])
    }

    async fn fetch_events_since(
        &self,
        calendar_id: &str,
        _since_utc: &str,
    ) -> Result<Vec<RawEvent>> {
        self.fetch_events(calendar_id).await
    }

    async fn sync_delta(&self, _calendar_id: &str, _sync_state: Option<&str>) -> Result<DeltaResult> {
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
}
