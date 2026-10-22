//! Duplicate an event type (Cascade fork; upstream issue #235 asks for the
//! same feature and has no implementation yet).
//!
//! The copy carries every column of the source row and every settings row
//! that hangs off it, so it books exactly like the original. It gets a
//! "(copy)" title, the first free `-copy` slug, and starts disabled: it is
//! not listed or bookable until the host has renamed it and switched it on.
//! Bookings and invite links are guest records, not settings, and stay with
//! the original.

use super::*;

/// Tables keyed by `event_type_id` that hold an event type's settings. Each
/// is copied row for row onto the new event type.
pub(super) const DUPLICATED_CHILD_TABLES: &[&str] = &[
    "availability_rules",
    "availability_overrides",
    "event_type_calendars",
    "event_type_member_weights",
    "event_type_watchers",
    "booking_frequency_limits",
    "event_type_resources",
];

/// Tables keyed by `event_type_id` that the copy deliberately leaves behind.
/// A guard test fails when a migration adds a table to neither list.
pub(super) const NOT_DUPLICATED_TABLES: &[&str] = &["bookings", "booking_invites"];

/// `event_types` columns the copy sets itself instead of carrying over.
const RESET_COLUMNS: &[&str] = &[
    "id",
    "slug",
    "title",
    "enabled",
    "created_at",
    "created_by_user_id",
];

/// Longest title the event type forms accept.
const MAX_TITLE: usize = 255;

/// Copies after the first are numbered from here: `-copy`, `-copy-2`, ...
const MAX_COPY_SUFFIX: u32 = 1000;

async fn table_columns(
    conn: &mut sqlx::SqliteConnection,
    table: &str,
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT name FROM pragma_table_info(?) ORDER BY cid")
        .bind(table)
        .fetch_all(conn)
        .await
}

fn quoted(columns: &[String]) -> String {
    columns
        .iter()
        .map(|c| format!("\"{}\"", c.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The first `-copy` slug not already used where it would collide: the
/// owning account (the table's unique key), the team, or a legacy group.
async fn free_copy_slug(
    conn: &mut sqlx::SqliteConnection,
    slug: &str,
    account_id: &str,
    team_id: Option<&str>,
    group_id: Option<&str>,
) -> Result<Option<String>, sqlx::Error> {
    for n in 1..=MAX_COPY_SUFFIX {
        let candidate = if n == 1 {
            format!("{slug}-copy")
        } else {
            format!("{slug}-copy-{n}")
        };
        let taken: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM event_types WHERE slug = ?
                 AND (account_id = ? OR team_id = ? OR group_id = ?))",
        )
        .bind(&candidate)
        .bind(account_id)
        .bind(team_id)
        .bind(group_id)
        .fetch_one(&mut *conn)
        .await?;
        if !taken {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}

/// "<title> (copy)" in the host's language, shortening the original title
/// when needed so the copy still saves through the edit form.
fn copy_title(lang: &str, title: &str) -> String {
    let render = |t: &str| {
        let mut args = fluent_bundle::FluentArgs::new();
        args.set("title", t.to_string());
        crate::i18n::translate(lang, "event-types-duplicate-title", Some(&args))
    };
    let full = render(title);
    let overflow = full.chars().count().saturating_sub(MAX_TITLE);
    if overflow == 0 {
        return full;
    }
    let keep = title.chars().count().saturating_sub(overflow);
    render(title.chars().take(keep).collect::<String>().trim_end())
}

/// Copy event type `source_id` with all its settings rows in one
/// transaction. Returns the new slug, or `None` when no copy slug is free.
pub(super) async fn duplicate_event_type_rows(
    pool: &SqlitePool,
    source_id: &str,
    user_id: &str,
    lang: &str,
) -> Result<Option<String>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (slug, title, account_id, team_id, group_id): (
        String,
        String,
        String,
        Option<String>,
        Option<String>,
    ) = sqlx::query_as(
        "SELECT slug, title, account_id, team_id, group_id FROM event_types WHERE id = ?",
    )
    .bind(source_id)
    .fetch_one(&mut *tx)
    .await?;

    let Some(new_slug) = free_copy_slug(
        &mut tx,
        &slug,
        &account_id,
        team_id.as_deref(),
        group_id.as_deref(),
    )
    .await?
    else {
        return Ok(None);
    };
    let new_id = uuid::Uuid::new_v4().to_string();

    // Every column the schema has, so settings added by later migrations
    // are copied without touching this code.
    let carried: Vec<String> = table_columns(&mut tx, "event_types")
        .await?
        .into_iter()
        .filter(|c| !RESET_COLUMNS.contains(&c.as_str()))
        .collect();
    let cols = quoted(&carried);
    sqlx::query(&format!(
        "INSERT INTO event_types (id, slug, title, enabled, created_by_user_id, {cols})
         SELECT ?, ?, ?, 0, ?, {cols} FROM event_types WHERE id = ?"
    ))
    .bind(&new_id)
    .bind(&new_slug)
    .bind(copy_title(lang, &title))
    .bind(user_id)
    .bind(source_id)
    .execute(&mut *tx)
    .await?;

    for table in DUPLICATED_CHILD_TABLES {
        let columns = table_columns(&mut tx, table).await?;
        let others: Vec<String> = columns
            .iter()
            .filter(|c| *c != "id" && *c != "event_type_id")
            .cloned()
            .collect();
        let rest = if others.is_empty() {
            String::new()
        } else {
            format!(", {}", quoted(&others))
        };
        if columns.iter().any(|c| c == "id") {
            // Rows with their own id get a fresh one each.
            let ids: Vec<String> =
                sqlx::query_scalar(&format!("SELECT id FROM {table} WHERE event_type_id = ?"))
                    .bind(source_id)
                    .fetch_all(&mut *tx)
                    .await?;
            for row_id in ids {
                sqlx::query(&format!(
                    "INSERT INTO {table} (id, event_type_id{rest})
                     SELECT ?, ?{rest} FROM {table} WHERE id = ?"
                ))
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&new_id)
                .bind(&row_id)
                .execute(&mut *tx)
                .await?;
            }
        } else {
            sqlx::query(&format!(
                "INSERT INTO {table} (event_type_id{rest})
                 SELECT ?{rest} FROM {table} WHERE event_type_id = ?"
            ))
            .bind(&new_id)
            .bind(source_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    tracing::info!(source_event_type_id = %source_id, event_type_id = %new_id, "event type duplicated");
    Ok(Some(new_slug))
}

fn duplicate_response(
    state: &AppState,
    auth_user: &crate::auth::AuthUser,
    result: Result<Option<String>, sqlx::Error>,
    edit_url: impl FnOnce(&str) -> String,
) -> axum::response::Response {
    match result {
        Ok(Some(slug)) => Redirect::to(&edit_url(&slug)).into_response(),
        Ok(None) => render_error_page_keys_in(
            state,
            auth_user.lang,
            axum::http::StatusCode::CONFLICT,
            "event-types-duplicate-failed-title",
            "event-types-duplicate-no-slug",
        ),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            internal_error_html("event type duplicate", &e),
        )
            .into_response(),
    }
}

fn event_type_not_found(state: &AppState, auth_user: &crate::auth::AuthUser) -> Response {
    render_error_page_keys_in(
        state,
        auth_user.lang,
        axum::http::StatusCode::NOT_FOUND,
        "error-title-not-found",
        "error-event-type-not-found",
    )
}

/// POST /dashboard/event-types/{slug}/duplicate. Same scope as the personal
/// edit route: the account owner's personal event type with that slug.
pub(super) async fn duplicate_event_type(
    State(state): State<Arc<AppState>>,
    auth_user: crate::auth::AuthUser,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Form(csrf): Form<CsrfForm>,
) -> impl IntoResponse {
    if let Err(resp) = verify_csrf_token(&headers, &csrf._csrf) {
        return resp;
    }
    let source: Option<String> = sqlx::query_scalar(
        "SELECT et.id FROM event_types et
         JOIN accounts a ON a.id = et.account_id
         WHERE a.user_id = ? AND et.slug = ? AND et.team_id IS NULL",
    )
    .bind(&auth_user.user.id)
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None);
    let Some(source_id) = source else {
        return event_type_not_found(&state, &auth_user);
    };

    let result =
        duplicate_event_type_rows(&state.pool, &source_id, &auth_user.user.id, auth_user.lang)
            .await;
    duplicate_response(&state, &auth_user, result, |s| {
        format!("/dashboard/event-types/{s}/edit")
    })
}

/// POST /dashboard/group-event-types/{team_id}/{slug}/duplicate. Same scope
/// as the team edit route: global admins, or admins of that team.
pub(super) async fn duplicate_group_event_type(
    State(state): State<Arc<AppState>>,
    auth_user: crate::auth::AuthUser,
    headers: HeaderMap,
    Path((team_id, slug)): Path<(String, String)>,
    Form(csrf): Form<CsrfForm>,
) -> impl IntoResponse {
    if let Err(resp) = verify_csrf_token(&headers, &csrf._csrf) {
        return resp;
    }
    let user = &auth_user.user;
    let source: Option<String> = if user.role == "admin" {
        sqlx::query_scalar("SELECT id FROM event_types WHERE team_id = ? AND slug = ?")
            .bind(&team_id)
            .bind(&slug)
            .fetch_optional(&state.pool)
            .await
            .unwrap_or(None)
    } else {
        sqlx::query_scalar(
            "SELECT et.id FROM event_types et
             JOIN team_members tm ON tm.team_id = et.team_id
             WHERE tm.user_id = ? AND tm.role = 'admin' AND et.team_id = ? AND et.slug = ?",
        )
        .bind(&user.id)
        .bind(&team_id)
        .bind(&slug)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None)
    };
    let Some(source_id) = source else {
        return event_type_not_found(&state, &auth_user);
    };

    let result = duplicate_event_type_rows(&state.pool, &source_id, &user.id, auth_user.lang).await;
    duplicate_response(&state, &auth_user, result, |s| {
        format!("/dashboard/group-event-types/{team_id}/{s}/edit")
    })
}
