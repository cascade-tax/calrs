//! Microsoft 365 calendar provider backed by Microsoft Graph.
//!
//! Authentication uses delegated OAuth2 access so every connected source is
//! limited to the calendars the signed-in Microsoft user can access.

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::{Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize};

use crate::providers::{CalendarProvider, DeltaResult, RawEvent, RemoteCalendar};
use crate::utils::{extract_vevent_field, extract_vevent_tzid};

pub const API_BASE: &str = "https://graph.microsoft.com/v1.0";
const LOGIN_BASE: &str = "https://login.microsoftonline.com";
const SCOPES: &str = "openid profile email offline_access User.Read Calendars.Read";
const CALRS_UID_PROPERTY: &str =
    "String {4c36e9a0-4348-4fb8-a2d2-705e42aafb7d} Name CalrsBookingUid";
const MAX_PAGES: usize = 1000;

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct MicrosoftIdentity {
    pub mail: Option<String>,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

pub fn normalize_tenant(value: &str) -> Result<String> {
    let tenant = value.trim();
    let tenant = if tenant.is_empty() {
        "organizations"
    } else {
        tenant
    };
    if tenant.len() > 255
        || !tenant
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-'))
    {
        bail!("Microsoft tenant must be 'organizations', a tenant ID, or a tenant domain");
    }
    Ok(tenant.to_string())
}

pub fn build_auth_url(
    client_id: &str,
    tenant: &str,
    redirect_uri: &str,
    state: &str,
) -> Result<String> {
    let tenant = normalize_tenant(tenant)?;
    let mut url = Url::parse(&format!("{LOGIN_BASE}/{tenant}/oauth2/v2.0/authorize"))?;
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_mode", "query")
        .append_pair("scope", SCOPES)
        .append_pair("state", state)
        .append_pair("prompt", "select_account");
    Ok(url.into())
}

async fn token_request(tenant: &str, form: &[(&str, &str)]) -> Result<TokenResponse> {
    let tenant = normalize_tenant(tenant)?;
    let response = reqwest::Client::new()
        .post(format!("{LOGIN_BASE}/{tenant}/oauth2/v2.0/token"))
        .form(form)
        .send()
        .await
        .context("Microsoft token request failed")?;
    let status = response.status();
    let bytes = response.bytes().await?;
    if !status.is_success() {
        let detail = serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .and_then(|v| {
                v.get("error_description")
                    .or_else(|| v.get("error"))
                    .and_then(|s| s.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| status.to_string());
        bail!("Microsoft authorization failed: {detail}");
    }
    serde_json::from_slice(&bytes).context("Microsoft token response was invalid")
}

pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    tenant: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<TokenResponse> {
    token_request(
        tenant,
        &[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("grant_type", "authorization_code"),
            ("scope", SCOPES),
        ],
    )
    .await
}

pub async fn refresh_token(
    client_id: &str,
    client_secret: &str,
    tenant: &str,
    refresh_token: &str,
) -> Result<TokenResponse> {
    token_request(
        tenant,
        &[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
            ("scope", SCOPES),
        ],
    )
    .await
}

pub async fn fetch_identity(access_token: &str) -> Result<MicrosoftIdentity> {
    let response = reqwest::Client::new()
        .get(format!(
            "{API_BASE}/me?$select=mail,userPrincipalName,displayName"
        ))
        .bearer_auth(access_token)
        .send()
        .await?;
    parse_response(response).await
}

#[derive(Clone)]
pub struct GraphProvider {
    client: reqwest::Client,
    access_token: String,
}

impl GraphProvider {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .expect("reqwest client configuration is valid"),
            access_token: access_token.into(),
        }
    }

    async fn send(&self, method: Method, url: &str) -> Result<reqwest::Response> {
        self.client
            .request(method, url)
            .bearer_auth(&self.access_token)
            .header("Prefer", "outlook.timezone=\"UTC\", IdType=\"ImmutableId\"")
            .send()
            .await
            .with_context(|| format!("Microsoft Graph request failed: {url}"))
    }

    async fn get_page<T: DeserializeOwned>(&self, url: &str) -> Result<GraphCollection<T>> {
        let response = self.send(Method::GET, url).await?;
        parse_response(response).await
    }

    async fn get_all<T: DeserializeOwned>(&self, first_url: String) -> Result<Vec<T>> {
        let mut all = Vec::new();
        let mut next = Some(first_url);
        for _ in 0..MAX_PAGES {
            let Some(url) = next.take() else {
                return Ok(all);
            };
            if !url.starts_with(API_BASE) {
                bail!("Microsoft Graph returned an unexpected pagination URL");
            }
            let page: GraphCollection<T> = self.get_page(&url).await?;
            all.extend(page.value);
            next = page.next_link;
        }
        bail!("Microsoft Graph pagination exceeded {MAX_PAGES} pages")
    }

    async fn get_delta_all(&self, first_url: String) -> Result<(Vec<GraphEvent>, String)> {
        let mut all = Vec::new();
        let mut next = first_url;
        for _ in 0..MAX_PAGES {
            if !next.starts_with(API_BASE) {
                bail!("Microsoft Graph returned an unexpected delta URL");
            }
            let page: GraphCollection<GraphEvent> = self.get_page(&next).await?;
            all.extend(page.value);
            if let Some(delta_link) = page.delta_link {
                if !delta_link.starts_with(API_BASE) {
                    bail!("Microsoft Graph returned an unexpected delta URL");
                }
                return Ok((all, delta_link));
            }
            next = page.next_link.ok_or_else(|| {
                anyhow::anyhow!("Microsoft Graph delta response had no continuation link")
            })?;
        }
        bail!("Microsoft Graph delta pagination exceeded {MAX_PAGES} pages")
    }

    fn calendar_url(calendar_id: &str, suffix: &str) -> String {
        format!(
            "{API_BASE}/me/calendars/{}/{}",
            urlencoding::encode(calendar_id),
            suffix.trim_start_matches('/')
        )
    }

    async fn find_booking_events(&self, calendar_id: &str, uid: &str) -> Result<Vec<GraphEvent>> {
        let escaped_uid = uid.replace('\'', "''");
        let filter = format!(
            "singleValueExtendedProperties/Any(ep: ep/id eq '{CALRS_UID_PROPERTY}' and ep/value eq '{escaped_uid}')"
        );
        let mut url = Url::parse(&Self::calendar_url(calendar_id, "events"))?;
        url.query_pairs_mut()
            .append_pair("$select", "id")
            .append_pair("$filter", &filter);
        self.get_all(url.into()).await
    }
}

#[async_trait]
impl CalendarProvider for GraphProvider {
    async fn check_connection(&self) -> Result<bool> {
        let response = self
            .send(Method::GET, &format!("{API_BASE}/me?$select=id"))
            .await?;
        parse_response::<serde_json::Value>(response).await?;
        Ok(true)
    }

    async fn list_calendars(&self) -> Result<Vec<RemoteCalendar>> {
        let calendars: Vec<GraphCalendar> = self
            .get_all(format!("{API_BASE}/me/calendars?$select=id,name"))
            .await?;
        Ok(calendars
            .into_iter()
            .map(|calendar| RemoteCalendar {
                id: calendar.id,
                display_name: calendar.name,
                // Graph's `color` field is a named Outlook enum rather than a
                // CSS color, so leave this unset instead of rendering an
                // invalid value in the calendar picker.
                color: None,
                change_marker: None,
                sync_state: None,
            })
            .collect())
    }

    async fn fetch_events(&self, calendar_id: &str) -> Result<Vec<RawEvent>> {
        let since = (Utc::now() - chrono::Duration::days(90)).to_rfc3339();
        self.fetch_events_since(calendar_id, &since).await
    }

    async fn fetch_events_since(
        &self,
        calendar_id: &str,
        since_utc: &str,
    ) -> Result<Vec<RawEvent>> {
        let start = DateTime::parse_from_rfc3339(since_utc)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now() - chrono::Duration::days(90));
        let end = start + chrono::Duration::days(730);
        let mut url = Url::parse(&Self::calendar_url(calendar_id, "calendarView"))?;
        url.query_pairs_mut()
            .append_pair("startDateTime", &start.to_rfc3339())
            .append_pair("endDateTime", &end.to_rfc3339())
            .append_pair(
                "$select",
                "id,iCalUId,subject,bodyPreview,start,end,isAllDay,isCancelled,showAs,location,type,originalStart",
            )
            .append_pair("$top", "250");

        let events: Vec<GraphEvent> = self.get_all(url.into()).await?;
        Ok(events
            .into_iter()
            .filter_map(|event| {
                let remote_id = event.id.clone();
                synth_vcalendar(&event).map(|ical| RawEvent { remote_id, ical })
            })
            .collect())
    }

    async fn sync_delta(&self, calendar_id: &str, sync_state: Option<&str>) -> Result<DeltaResult> {
        let first_url = match sync_state {
            Some(link) => link.to_string(),
            None => {
                let start = Utc::now() - chrono::Duration::days(90);
                let end = start + chrono::Duration::days(730);
                let mut url = Url::parse(&Self::calendar_url(calendar_id, "calendarView/delta"))?;
                url.query_pairs_mut()
                    .append_pair("startDateTime", &start.to_rfc3339())
                    .append_pair("endDateTime", &end.to_rfc3339());
                url.into()
            }
        };
        let (events, delta_link) = self.get_delta_all(first_url).await?;
        let (added_or_changed, deleted_uids) = partition_delta_events(events);
        Ok(DeltaResult {
            added_or_changed,
            deleted_uids,
            new_sync_state: Some(delta_link),
        })
    }

    async fn put_event(&self, calendar_id: &str, uid: &str, ics: &str) -> Result<()> {
        let mut existing = self.find_booking_events(calendar_id, uid).await?;
        let base_payload = event_payload_from_ics(ics)?;

        if let Some(first) = existing.first() {
            let url = Self::calendar_url(
                calendar_id,
                &format!("events/{}", urlencoding::encode(&first.id)),
            );
            let response = self
                .client
                .patch(url)
                .bearer_auth(&self.access_token)
                .json(&base_payload)
                .send()
                .await?;
            parse_empty_or_json(response).await?;
            existing.remove(0);
        } else {
            let mut payload = base_payload;
            payload["transactionId"] = serde_json::Value::String(uid.to_string());
            payload["singleValueExtendedProperties"] = serde_json::json!([{
                "id": CALRS_UID_PROPERTY,
                "value": uid,
            }]);
            let response = self
                .client
                .post(Self::calendar_url(calendar_id, "events"))
                .bearer_auth(&self.access_token)
                .json(&payload)
                .send()
                .await?;
            parse_response::<serde_json::Value>(response).await?;
        }

        // Repair any duplicates left by a prior interrupted create attempt.
        for duplicate in existing {
            let url = Self::calendar_url(
                calendar_id,
                &format!("events/{}", urlencoding::encode(&duplicate.id)),
            );
            let response = self.send(Method::DELETE, &url).await?;
            parse_empty_or_json(response).await?;
        }
        Ok(())
    }

    async fn delete_event(&self, calendar_id: &str, uid: &str) -> Result<()> {
        for event in self.find_booking_events(calendar_id, uid).await? {
            let url = Self::calendar_url(
                calendar_id,
                &format!("events/{}", urlencoding::encode(&event.id)),
            );
            let response = self.send(Method::DELETE, &url).await?;
            if response.status() != StatusCode::NOT_FOUND {
                parse_empty_or_json(response).await?;
            }
        }
        Ok(())
    }
}

fn partition_delta_events(events: Vec<GraphEvent>) -> (Vec<RawEvent>, Vec<String>) {
    let mut added_or_changed = Vec::new();
    let mut deleted = Vec::new();
    for event in events {
        if event.removed.is_some() {
            deleted.push(event.id);
        } else {
            let remote_id = event.id.clone();
            if let Some(ical) = synth_vcalendar(&event) {
                added_or_changed.push(RawEvent { remote_id, ical });
            }
        }
    }
    (added_or_changed, deleted)
}

#[derive(Debug, Deserialize)]
struct GraphCollection<T> {
    value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
    #[serde(rename = "@odata.deltaLink")]
    delta_link: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GraphCalendar {
    id: String,
    name: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct GraphEvent {
    id: String,
    #[serde(rename = "iCalUId")]
    i_cal_uid: Option<String>,
    subject: Option<String>,
    #[serde(rename = "bodyPreview")]
    body_preview: Option<String>,
    start: Option<GraphDateTime>,
    end: Option<GraphDateTime>,
    #[serde(rename = "isAllDay", default)]
    is_all_day: bool,
    #[serde(rename = "isCancelled", default)]
    is_cancelled: bool,
    #[serde(rename = "showAs")]
    show_as: Option<String>,
    location: Option<GraphLocation>,
    #[serde(rename = "type")]
    event_type: Option<String>,
    #[serde(rename = "originalStart")]
    original_start: Option<String>,
    #[serde(rename = "@removed")]
    removed: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GraphDateTime {
    #[serde(rename = "dateTime")]
    date_time: String,
    #[serde(rename = "timeZone")]
    time_zone: String,
}

#[derive(Debug, Deserialize)]
struct GraphLocation {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

async fn parse_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let bytes = response.bytes().await?;
    if !status.is_success() {
        bail!("Microsoft Graph returned {status}: {}", graph_error(&bytes));
    }
    serde_json::from_slice(&bytes).context("Microsoft Graph returned invalid JSON")
}

async fn parse_empty_or_json(response: reqwest::Response) -> Result<()> {
    let status = response.status();
    let bytes = response.bytes().await?;
    if !status.is_success() {
        bail!("Microsoft Graph returned {status}: {}", graph_error(&bytes));
    }
    Ok(())
}

fn graph_error(bytes: &[u8]) -> String {
    serde_json::from_slice::<serde_json::Value>(bytes)
        .ok()
        .and_then(|v| {
            let error = v.get("error")?;
            let code = error.get("code").and_then(|v| v.as_str()).unwrap_or("");
            let message = error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("request failed");
            Some(if code.is_empty() {
                message.to_string()
            } else {
                format!("{code}: {message}")
            })
        })
        .unwrap_or_else(|| "request failed".to_string())
}

fn synth_vcalendar(event: &GraphEvent) -> Option<String> {
    let uid = event
        .i_cal_uid
        .as_deref()
        .filter(|uid| !uid.is_empty())
        .unwrap_or(&event.id);
    let start = graph_datetime_to_ical(event.start.as_ref()?)?;
    let end = graph_datetime_to_ical(event.end.as_ref()?)?;
    let mut out = String::from(
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Cascade//Microsoft Graph//EN\r\nBEGIN:VEVENT\r\n",
    );
    out.push_str(&format!("UID:{}\r\n", escape_ical_text(uid)));
    out.push_str(&format!(
        "DTSTAMP:{}\r\n",
        Utc::now().format("%Y%m%dT%H%M%SZ")
    ));
    out.push_str(&format!("DTSTART:{start}\r\nDTEND:{end}\r\n"));
    if let Some(subject) = event.subject.as_deref().filter(|v| !v.is_empty()) {
        out.push_str(&format!("SUMMARY:{}\r\n", escape_ical_text(subject)));
    }
    if let Some(description) = event.body_preview.as_deref().filter(|v| !v.is_empty()) {
        out.push_str(&format!(
            "DESCRIPTION:{}\r\n",
            escape_ical_text(description)
        ));
    }
    if let Some(location) = event
        .location
        .as_ref()
        .and_then(|v| v.display_name.as_deref())
        .filter(|v| !v.is_empty())
    {
        out.push_str(&format!("LOCATION:{}\r\n", escape_ical_text(location)));
    }
    if matches!(
        event.event_type.as_deref(),
        Some("occurrence" | "exception")
    ) {
        let recurrence_id = event
            .original_start
            .as_deref()
            .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
            .map(|value| {
                value
                    .with_timezone(&Utc)
                    .format("%Y%m%dT%H%M%SZ")
                    .to_string()
            })
            .unwrap_or_else(|| start.clone());
        out.push_str(&format!("RECURRENCE-ID:{recurrence_id}\r\n"));
    }
    let transparent = matches!(event.show_as.as_deref(), Some("free"));
    out.push_str(if transparent {
        "TRANSP:TRANSPARENT\r\n"
    } else {
        "TRANSP:OPAQUE\r\n"
    });
    out.push_str(if event.is_cancelled {
        "STATUS:CANCELLED\r\n"
    } else {
        "STATUS:CONFIRMED\r\n"
    });
    if event.is_all_day {
        out.push_str("X-MICROSOFT-CDO-ALLDAYEVENT:TRUE\r\n");
    }
    out.push_str("END:VEVENT\r\nEND:VCALENDAR\r\n");
    Some(out)
}

fn graph_datetime_to_ical(value: &GraphDateTime) -> Option<String> {
    let raw = value.date_time.trim_end_matches('Z');
    let parsed = NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S%.f")
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S"))
        .ok()?;
    let suffix = if value.date_time.ends_with('Z') || value.time_zone.eq_ignore_ascii_case("UTC") {
        "Z"
    } else {
        ""
    };
    Some(format!("{}{suffix}", parsed.format("%Y%m%dT%H%M%S")))
}

fn event_payload_from_ics(ics: &str) -> Result<serde_json::Value> {
    let start_raw = extract_vevent_field(ics, "DTSTART")
        .ok_or_else(|| anyhow::anyhow!("iCalendar event is missing DTSTART"))?;
    let end_raw = extract_vevent_field(ics, "DTEND")
        .ok_or_else(|| anyhow::anyhow!("iCalendar event is missing DTEND"))?;
    let start = ical_datetime_to_graph(&start_raw)?;
    let end = ical_datetime_to_graph(&end_raw)?;
    let timezone = extract_vevent_tzid(ics, "DTSTART").unwrap_or_else(|| "UTC".to_string());
    let subject = extract_vevent_field(ics, "SUMMARY")
        .map(|v| unescape_ical_text(&v))
        .unwrap_or_else(|| "Booked meeting".to_string());
    let description = extract_vevent_field(ics, "DESCRIPTION")
        .map(|v| unescape_ical_text(&v))
        .unwrap_or_default();
    let location = extract_vevent_field(ics, "LOCATION")
        .map(|v| unescape_ical_text(&v))
        .unwrap_or_default();

    Ok(serde_json::json!({
        "subject": subject,
        "body": { "contentType": "text", "content": description },
        "start": { "dateTime": start, "timeZone": timezone },
        "end": { "dateTime": end, "timeZone": timezone },
        "location": { "displayName": location },
        "showAs": "busy",
    }))
}

fn ical_datetime_to_graph(value: &str) -> Result<String> {
    let raw = value.trim_end_matches('Z');
    let parsed = NaiveDateTime::parse_from_str(raw, "%Y%m%dT%H%M%S")
        .or_else(|_| NaiveDateTime::parse_from_str(raw, "%Y%m%dT%H%M"))
        .with_context(|| format!("Unsupported iCalendar date-time: {value}"))?;
    Ok(parsed.format("%Y-%m-%dT%H:%M:%S").to_string())
}

fn escape_ical_text(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "")
        .replace(',', "\\,")
        .replace(';', "\\;")
}

fn unescape_ical_text(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n' | 'N') => output.push('\n'),
                Some(next) => output.push(next),
                None => output.push('\\'),
            }
        } else {
            output.push(ch);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn microsoft_auth_url_uses_organizational_scopes_and_encodes_state() {
        let url = build_auth_url(
            "client-id",
            "organizations",
            "https://calendar.example.test/dashboard/sources/microsoft/callback",
            "state with & symbols",
        )
        .unwrap();
        let parsed = Url::parse(&url).unwrap();
        let query: std::collections::HashMap<_, _> = parsed.query_pairs().collect();
        assert_eq!(parsed.path(), "/organizations/oauth2/v2.0/authorize");
        assert_eq!(query.get("response_type").map(|v| v.as_ref()), Some("code"));
        assert_eq!(
            query.get("state").map(|v| v.as_ref()),
            Some("state with & symbols")
        );
        let scope = query.get("scope").expect("scope query parameter");
        let scopes: std::collections::HashSet<_> = scope.split_whitespace().collect();
        assert!(scopes.contains("Calendars.Read"));
        assert!(scopes.contains("offline_access"));
        assert!(!scopes.iter().any(|scope| scope.contains("ReadWrite")));
    }

    #[test]
    fn tenant_rejects_url_path_injection() {
        assert!(normalize_tenant("organizations/oauth2").is_err());
        assert_eq!(normalize_tenant("").unwrap(), "organizations");
        assert!(normalize_tenant("contoso.onmicrosoft.com").is_ok());
    }

    #[test]
    fn graph_event_synthesizes_busy_ical() {
        let event: GraphEvent = serde_json::from_value(serde_json::json!({
            "id": "graph-id",
            "iCalUId": "ical-uid",
            "subject": "Planning, review",
            "bodyPreview": "Line one\nLine two",
            "start": { "dateTime": "2026-08-20T16:00:00.0000000", "timeZone": "UTC" },
            "end": { "dateTime": "2026-08-20T16:30:00.0000000", "timeZone": "UTC" },
            "showAs": "busy",
            "location": { "displayName": "Room 1" },
            "type": "singleInstance"
        }))
        .unwrap();
        let ical = synth_vcalendar(&event).unwrap();
        assert!(ical.contains("UID:ical-uid"));
        assert!(ical.contains("DTSTART:20260820T160000Z"));
        assert!(ical.contains("SUMMARY:Planning\\, review"));
        assert!(ical.contains("TRANSP:OPAQUE"));
    }

    #[test]
    fn moved_exception_keeps_original_occurrence_as_recurrence_id() {
        let event: GraphEvent = serde_json::from_value(serde_json::json!({
            "id": "graph-exception-id",
            "iCalUId": "series-uid",
            "originalStart": "2026-08-20T16:00:00Z",
            "start": { "dateTime": "2026-08-20T18:00:00", "timeZone": "UTC" },
            "end": { "dateTime": "2026-08-20T18:30:00", "timeZone": "UTC" },
            "type": "exception"
        }))
        .unwrap();
        let ical = synth_vcalendar(&event).unwrap();
        assert!(ical.contains("DTSTART:20260820T180000Z"));
        assert!(ical.contains("RECURRENCE-ID:20260820T160000Z"));
    }

    #[test]
    fn write_payload_uses_utc_and_omits_attendees() {
        let ics = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nUID:booking-1\r\nDTSTART:20260820T160000Z\r\nDTEND:20260820T163000Z\r\nSUMMARY:Review\\, weekly\r\nLOCATION:Room 1\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let payload = event_payload_from_ics(ics).unwrap();
        assert_eq!(payload["start"]["dateTime"], "2026-08-20T16:00:00");
        assert_eq!(payload["start"]["timeZone"], "UTC");
        assert_eq!(payload["subject"], "Review, weekly");
        assert!(payload.get("attendees").is_none());
    }

    #[test]
    fn delta_tombstones_are_partitioned_by_remote_id() {
        let events: Vec<GraphEvent> = serde_json::from_value(serde_json::json!([
            {
                "id": "deleted-graph-id",
                "@removed": { "reason": "deleted" }
            },
            {
                "id": "active-graph-id",
                "iCalUId": "active-ical-uid",
                "start": { "dateTime": "2026-08-20T16:00:00", "timeZone": "UTC" },
                "end": { "dateTime": "2026-08-20T16:30:00", "timeZone": "UTC" }
            }
        ]))
        .unwrap();
        let (changed, deleted) = partition_delta_events(events);
        assert_eq!(deleted, vec!["deleted-graph-id"]);
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].remote_id, "active-graph-id");
    }
}
