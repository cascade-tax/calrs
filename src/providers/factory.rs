//! Construct a [`CalendarProvider`] from a `caldav_sources` row.
//!
//! Centralising the dispatch here keeps the rest of the codebase ignorant of
//! which protocol a source uses. Add a new back-end by extending the match in
//! `build_provider`.

use anyhow::{bail, Result};
use sqlx::SqlitePool;

use super::CalendarProvider;

/// Provider type stored in `caldav_sources.provider_type`.
pub mod kinds {
    pub const CALDAV: &str = "caldav";
    pub const EWS: &str = "ews";
    pub const MICROSOFT_GRAPH: &str = "microsoft_graph";
    pub const PUBLISHED_ICS: &str = "published_ics";
}

/// Build a provider client for the given source row.
///
/// `provider_type` is the value stored in `caldav_sources.provider_type`. The
/// other parameters are the URL / username / decrypted password — any of them
/// may carry provider-specific meaning (e.g. for EWS the URL is the
/// `Exchange.asmx` endpoint, for CalDAV it is the discovery URL).
pub fn build_provider(
    provider_type: &str,
    url: &str,
    username: &str,
    password: &str,
) -> Result<Box<dyn CalendarProvider>> {
    match provider_type {
        kinds::CALDAV => Ok(Box::new(super::caldav::CaldavProvider::new(
            url, username, password,
        ))),
        kinds::EWS => Ok(Box::new(crate::ews::EwsProvider::new(
            url, username, password,
        ))),
        kinds::MICROSOFT_GRAPH => Ok(Box::new(crate::microsoft_graph::GraphProvider::new(
            password,
        ))),
        kinds::PUBLISHED_ICS => Ok(Box::new(
            super::published_ics::PublishedIcsProvider::new(password),
        )),
        other => bail!("Unknown calendar provider type: '{}'", other),
    }
}

/// Build any persisted source, decrypting its credential and refreshing an
/// OAuth2 token when required. This keeps sync, test, and write-back paths on
/// the same authentication behavior.
#[allow(clippy::too_many_arguments)]
pub async fn build_provider_for_source(
    pool: &SqlitePool,
    key: &[u8; 32],
    source_id: &str,
    provider_type: &str,
    url: &str,
    username: &str,
    password_enc: Option<&str>,
    auth_type: &str,
    access_token_enc: Option<&str>,
    token_expires_at: Option<&str>,
) -> Result<Box<dyn CalendarProvider>> {
    match provider_type {
        kinds::CALDAV => {
            let client = crate::oauth2_caldav::build_client_for_source(
                pool,
                key,
                source_id,
                url,
                auth_type,
                username,
                password_enc,
                access_token_enc,
                token_expires_at,
            )
            .await?;
            Ok(Box::new(super::caldav::CaldavProvider::from_client(client)))
        }
        kinds::EWS => {
            let encrypted =
                password_enc.ok_or_else(|| anyhow::anyhow!("Exchange source missing password"))?;
            let password = crate::crypto::decrypt_password(key, encrypted)?;
            build_provider(provider_type, url, username, &password)
        }
        kinds::MICROSOFT_GRAPH => {
            if auth_type != "oauth2" {
                bail!("Microsoft 365 source is missing OAuth2 authentication");
            }
            let encrypted = access_token_enc
                .ok_or_else(|| anyhow::anyhow!("Microsoft 365 source missing access token"))?;
            let access_token = crate::oauth2_caldav::get_valid_access_token(
                pool,
                key,
                source_id,
                encrypted,
                token_expires_at,
            )
            .await?;
            build_provider(provider_type, url, username, &access_token)
        }
        kinds::PUBLISHED_ICS => {
            let encrypted = password_enc
                .ok_or_else(|| anyhow::anyhow!("Published calendar source is missing its URL"))?;
            let feed_url = crate::crypto::decrypt_password(key, encrypted)?;
            build_provider(provider_type, url, username, &feed_url)
        }
        other => bail!("Unknown calendar provider type: '{}'", other),
    }
}

/// Validate a URL based on the provider type. CalDAV and EWS both reject
/// non-http(s) and SSRF-prone hostnames.
pub fn validate_url(provider_type: &str, url: &str) -> Result<()> {
    match provider_type {
        kinds::CALDAV | kinds::EWS => crate::caldav::validate_caldav_url(url),
        kinds::PUBLISHED_ICS => {
            super::published_ics::validate_subscription_url(url).map(|_| ())
        }
        kinds::MICROSOFT_GRAPH if url == crate::microsoft_graph::API_BASE => Ok(()),
        kinds::MICROSOFT_GRAPH => bail!("Microsoft Graph API URL is not configurable"),
        other => bail!("Unknown calendar provider type: '{}'", other),
    }
}

/// Human-readable label for UI listings.
pub fn label(provider_type: &str) -> &'static str {
    match provider_type {
        kinds::CALDAV => "CalDAV",
        kinds::EWS => "Microsoft Exchange (EWS)",
        kinds::MICROSOFT_GRAPH => "Microsoft 365",
        kinds::PUBLISHED_ICS => "Published ICS",
        _ => "Unknown",
    }
}

/// Whether a source can be selected as a booking write-back target.
///
/// Microsoft Graph is intentionally read-only: its OAuth grant requests
/// `Calendars.Read`, so exposing a write target would create a configuration
/// that can never succeed.
pub fn supports_write_back(provider_type: &str) -> bool {
    !matches!(
        provider_type,
        kinds::MICROSOFT_GRAPH | kinds::PUBLISHED_ICS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn microsoft_graph_is_read_only() {
        assert!(!supports_write_back(kinds::MICROSOFT_GRAPH));
        assert!(!supports_write_back(kinds::PUBLISHED_ICS));
        assert!(supports_write_back(kinds::CALDAV));
        assert!(supports_write_back(kinds::EWS));
    }
}
