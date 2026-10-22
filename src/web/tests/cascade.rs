//! Cascade-fork behaviour: Sunday-first weeks, the 12-hour clock, the
//! Cascade palette and branding, styled error pages, team conflict
//! calendars, collective troubleshooting and attendees, published
//! calendar sources, and duplicating event types. Kept in one file so an
//! upstream port can see at a glance what the fork guarantees.

use super::*;

#[test]
fn month_grid_uses_sunday_as_first_day() {
    // August 1, 2026 is a Saturday: its Sunday-first grid offset is 6.
    let params = build_month_params(2026, 8, chrono_tz::UTC, chrono_tz::UTC, "en", None);
    assert_eq!(params.5, 6);

    // November 1, 2026 is a Sunday and therefore starts at column 0.
    let params = build_month_params(2026, 11, chrono_tz::UTC, chrono_tz::UTC, "en", None);
    assert_eq!(params.5, 0);
}

#[test]
fn weekly_booking_limits_use_sunday_boundaries() {
    let monday = NaiveDate::from_ymd_opt(2026, 8, 17)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();
    let (start, end) = frequency_period_range(monday, "week");
    assert_eq!(start.date(), NaiveDate::from_ymd_opt(2026, 8, 16).unwrap());
    assert_eq!(end.date(), NaiveDate::from_ymd_opt(2026, 8, 23).unwrap());
}

#[tokio::test]
async fn event_type_calendar_filter_is_scoped_to_calendar_owner() {
    let pool = setup_test_db().await;
    let (owner_id, owner_account_id, et_id) = seed_test_data(&pool).await;
    let member_id = insert_role_user(&pool, "member@x", "user").await;
    let _ = insert_personal_et(&pool, &member_id, "member-personal").await;
    let member_account_id: String = sqlx::query_scalar("SELECT id FROM accounts WHERE user_id = ?")
        .bind(&member_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let selected_owner_calendar = seed_synced_event(
        &pool,
        &owner_account_id,
        "20300107T100000",
        "20300107T110000",
        "UTC",
        "OPAQUE",
    )
    .await;
    seed_synced_event(
        &pool,
        &owner_account_id,
        "20300107T120000",
        "20300107T130000",
        "UTC",
        "OPAQUE",
    )
    .await;
    seed_synced_event(
        &pool,
        &member_account_id,
        "20300107T140000",
        "20300107T150000",
        "UTC",
        "OPAQUE",
    )
    .await;
    sqlx::query("INSERT INTO event_type_calendars (event_type_id, calendar_id) VALUES (?, ?)")
        .bind(&et_id)
        .bind(&selected_owner_calendar)
        .execute(&pool)
        .await
        .unwrap();

    let owner_busy = fetch_busy_times_for_user(
        &pool,
        &owner_id,
        dt(2030, 1, 7, 0, 0),
        dt(2030, 1, 7, 23, 59),
        Tz::UTC,
        Some(&et_id),
    )
    .await;
    let member_busy = fetch_busy_times_for_user(
        &pool,
        &member_id,
        dt(2030, 1, 7, 0, 0),
        dt(2030, 1, 7, 23, 59),
        Tz::UTC,
        Some(&et_id),
    )
    .await;

    assert_eq!(
        owner_busy,
        vec![(dt(2030, 1, 7, 10, 0), dt(2030, 1, 7, 11, 0))],
        "the owner's allowlist should exclude their unselected calendar"
    );
    assert_eq!(
        member_busy,
        vec![(dt(2030, 1, 7, 14, 0), dt(2030, 1, 7, 15, 0))],
        "one member's allowlist must not exclude another member's calendars"
    );
}

#[test]
fn cascade_custom_palette_defers_to_the_built_in_design_system() {
    // Cascade is the built-in default in base.html, including the coral
    // accent and heading tokens a five-input custom theme cannot express.
    // Selecting it as a "custom" theme must therefore emit no overrides at
    // all rather than a second, drifting copy of the palette.
    let css = custom_theme_css("#1A3A4A", "#2A5A6A", "#F5F3F0", "#FFFFFF", "#22201D");
    assert_eq!(css, "");

    // A neighbouring palette is still derived normally.
    let other = custom_theme_css("#1A3A4B", "#2A5A6A", "#F5F3F0", "#FFFFFF", "#22201D");
    assert!(other.contains("--accent:#1A3A4B"));
}

#[tokio::test]
async fn build_theme_css_is_empty_when_custom_colors_are_unset() {
    // theme='custom' with the columns still NULL falls back to the Cascade
    // palette, which base.html already defines in full (including the
    // coral, heading and surface tokens five inputs cannot reconstruct).
    // Emitting nothing is the contract: one definition, no drift.
    let pool = setup_test_db().await;
    sqlx::query("UPDATE auth_config SET theme = 'custom' WHERE id = 'singleton'")
        .execute(&pool)
        .await
        .unwrap();

    let css = build_theme_css(&pool).await;
    assert!(
        css.is_empty(),
        "the Cascade palette is the built-in default and must not be \
         re-emitted as an override: {css}"
    );
}

#[test]
fn embed_brand_cannot_override_cascade_palette() {
    let p = EmbedParams {
        brand: Some("ff0000".to_string()),
        ..Default::default()
    };
    assert!(p.brand_css().is_none());
}

async fn capture_calendar_put(
    State(captured): State<std::sync::Arc<std::sync::Mutex<Vec<String>>>>,
    body: String,
) -> axum::http::StatusCode {
    captured.lock().unwrap().push(body);
    axum::http::StatusCode::CREATED
}

#[tokio::test]
async fn troubleshoot_includes_collective_team_event_types() {
    let (app, pool, session, et_id) = setup_test_app().await;
    let user_id: String = sqlx::query_scalar("SELECT id FROM users WHERE username = 'testuser'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let owner_account_id: String = sqlx::query_scalar("SELECT id FROM accounts WHERE user_id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let member_id = uuid::Uuid::new_v4().to_string();
    let member_account_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO users (id, email, name, role, auth_provider, username, enabled, timezone)
         VALUES (?, 'member@example.com', 'Other Member', 'user', 'local', 'other-member', 1, 'UTC')",
    )
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO accounts (id, name, email, timezone, user_id)
         VALUES (?, 'Other Member', 'member@example.com', 'UTC', ?)",
    )
    .bind(&member_account_id)
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_availability_rules (id, user_id, day_of_week, start_time, end_time)
         VALUES (?, ?, 1, '09:00', '12:00')",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO teams (id, name, slug, visibility, created_by) VALUES ('team-only', 'Team Only', 'team-only', 'public', ?)",
    )
    .bind(&user_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, source) VALUES ('team-only', ?, 'admin', 'direct')",
    )
    .bind(&user_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, source) VALUES ('team-only', ?, 'member', 'direct')",
    )
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE event_types SET team_id = 'team-only', scheduling_mode = 'collective'")
        .execute(&pool)
        .await
        .unwrap();
    let owner_calendar_id = seed_synced_event(
        &pool,
        &owner_account_id,
        "20300107T080000",
        "20300107T083000",
        "UTC",
        "OPAQUE",
    )
    .await;
    sqlx::query("INSERT INTO event_type_calendars (event_type_id, calendar_id) VALUES (?, ?)")
        .bind(&et_id)
        .bind(&owner_calendar_id)
        .execute(&pool)
        .await
        .unwrap();
    seed_synced_event(
        &pool,
        &member_account_id,
        "20300107T100000",
        "20300107T110000",
        "UTC",
        "OPAQUE",
    )
    .await;
    sqlx::query("UPDATE caldav_sources SET last_synced = datetime('now')")
        .execute(&pool)
        .await
        .unwrap();

    let response = app
        .oneshot(get_authed(
            &format!(
                "/dashboard/troubleshoot?event_type={}&date=2030-01-07",
                et_id
            ),
            &session,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_string(response).await;
    assert!(
        body.contains("Test Meeting — Team Only"),
        "collective team event type should appear in Troubleshoot"
    );
    assert!(
        body.contains("Other Member unavailable"),
        "a collective member's busy calendar should block the team timeline"
    );
    assert!(body.contains("Calendar conflict"));
    assert!(
        body.contains("Other Member outside working hours"),
        "a collective member's working hours should block the team timeline"
    );
    assert!(!body.contains("personal event types only"));
}

#[tokio::test]
async fn troubleshoot_explains_round_robin_only_event_types() {
    let (app, pool, session, _) = setup_test_app().await;
    let user_id: String = sqlx::query_scalar("SELECT id FROM users WHERE username = 'testuser'")
        .fetch_one(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO teams (id, name, slug, visibility, created_by)
         VALUES ('round-robin-only', 'Round Robin Only', 'round-robin-only', 'public', ?)",
    )
    .bind(&user_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, source)
         VALUES ('round-robin-only', ?, 'admin', 'direct')",
    )
    .bind(&user_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE event_types SET team_id = 'round-robin-only'")
        .execute(&pool)
        .await
        .unwrap();

    let response = app
        .oneshot(get_authed("/dashboard/troubleshoot", &session))
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_string(response).await;
    assert!(body.contains("Round-robin event types are not listed yet"));
    assert!(!body.contains("Test Meeting — Round Robin Only"));
}

#[tokio::test]
async fn team_member_updates_only_their_conflict_calendars() {
    let (app, pool, _owner_session, et_id) = setup_test_app().await;
    let owner_id: String = sqlx::query_scalar("SELECT id FROM users WHERE username = 'testuser'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let owner_account_id: String = sqlx::query_scalar("SELECT id FROM accounts WHERE user_id = ?")
        .bind(&owner_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let member_id = uuid::Uuid::new_v4().to_string();
    let member_account_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO users (id, email, name, role, auth_provider, username, enabled, timezone)
         VALUES (?, 'member@example.com', 'Team Member', 'user', 'local', 'team-member', 1, 'UTC')",
    )
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO accounts (id, name, email, timezone, user_id)
         VALUES (?, 'Team Member', 'member@example.com', 'UTC', ?)",
    )
    .bind(&member_account_id)
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    let member_session = uuid::Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::days(30))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    sqlx::query("INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(&member_session)
        .bind(&member_id)
        .bind(&expires_at)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO teams (id, name, slug, visibility, created_by)
         VALUES ('calendar-team', 'Calendar Team', 'calendar-team', 'public', ?)",
    )
    .bind(&owner_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, source)
         VALUES ('calendar-team', ?, 'admin', 'direct')",
    )
    .bind(&owner_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO team_members (team_id, user_id, role, source)
         VALUES ('calendar-team', ?, 'member', 'direct')",
    )
    .bind(&member_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "UPDATE event_types SET team_id = 'calendar-team', scheduling_mode = 'collective'
         WHERE id = ?",
    )
    .bind(&et_id)
    .execute(&pool)
    .await
    .unwrap();

    let owner_calendar = seed_synced_event(
        &pool,
        &owner_account_id,
        "20300107T080000",
        "20300107T083000",
        "UTC",
        "OPAQUE",
    )
    .await;
    let member_calendar_one = seed_synced_event(
        &pool,
        &member_account_id,
        "20300107T090000",
        "20300107T093000",
        "UTC",
        "OPAQUE",
    )
    .await;
    let member_calendar_two = seed_synced_event(
        &pool,
        &member_account_id,
        "20300107T100000",
        "20300107T103000",
        "UTC",
        "OPAQUE",
    )
    .await;
    sqlx::query("UPDATE calendars SET display_name = 'Owner Private' WHERE id = ?")
        .bind(&owner_calendar)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE calendars SET display_name = 'Member Work' WHERE id = ?")
        .bind(&member_calendar_one)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE calendars SET display_name = 'Member Personal' WHERE id = ?")
        .bind(&member_calendar_two)
        .execute(&pool)
        .await
        .unwrap();
    for calendar_id in [&owner_calendar, &member_calendar_one] {
        sqlx::query("INSERT INTO event_type_calendars (event_type_id, calendar_id) VALUES (?, ?)")
            .bind(&et_id)
            .bind(calendar_id)
            .execute(&pool)
            .await
            .unwrap();
    }

    let response = app
        .clone()
        .oneshot(get_authed(
            "/dashboard/group-event-types/calendar-team/test-meeting/calendars",
            &member_session,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body = body_string(response).await;
    assert!(body.contains("Member Work"));
    assert!(body.contains("Member Personal"));
    assert!(
        !body.contains("Owner Private"),
        "members must not see a teammate's calendar names"
    );

    let csrf = "team-calendar-csrf";
    let body = format!("_csrf={csrf}&calendar_ids={member_calendar_two}");
    let response = app
        .oneshot(post_form(
            "/dashboard/group-event-types/calendar-team/test-meeting/calendars",
            &member_session,
            csrf,
            &body,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), 303);

    let selected: Vec<String> = sqlx::query_scalar(
        "SELECT calendar_id FROM event_type_calendars WHERE event_type_id = ? ORDER BY calendar_id",
    )
    .bind(&et_id)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert!(selected.contains(&owner_calendar));
    assert!(!selected.contains(&member_calendar_one));
    assert!(selected.contains(&member_calendar_two));
    assert_eq!(selected.len(), 2);
}

#[tokio::test]
async fn approve_collective_booking_preserves_guest_added_and_cohost_attendees() {
    approve_collective_booking_and_capture("UTC", "2030-06-20T10:00:00", "2030-06-20T10:30:00", 0)
        .await;
}

#[tokio::test]
async fn approve_collective_utc_booking_preserves_attendees_and_exact_times() {
    let bodies = approve_collective_booking_and_capture(
        "America/Los_Angeles",
        "2030-06-20T17:00:00Z",
        "2030-06-20T17:30:00Z",
        1,
    )
    .await;
    for ics in &bodies {
        assert!(ics.contains("DTSTART:20300620T170000Z"), "{ics}");
        assert!(ics.contains("DTEND:20300620T173000Z"), "{ics}");
    }
}

async fn approve_collective_booking_and_capture(
    guest_timezone: &str,
    start_at: &str,
    end_at: &str,
    time_version: i64,
) -> Vec<String> {
    let (app, pool, _, et_id) = setup_test_app().await;

    let captured = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let capture_app = Router::new()
        .route("/{*path}", axum::routing::put(capture_calendar_put))
        .with_state(captured.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, capture_app).await.unwrap();
    });

    let organizer_id: String = sqlx::query_scalar(
        "SELECT a.user_id FROM event_types et JOIN accounts a ON a.id = et.account_id WHERE et.id = ?",
    )
    .bind(&et_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    let organizer_account_id: String =
        sqlx::query_scalar("SELECT account_id FROM event_types WHERE id = ?")
            .bind(&et_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    let cohost_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO users (id, email, name, role, auth_provider, username, enabled) VALUES (?, 'cohost@test.com', 'Zed Co-host', 'user', 'local', 'cohost', 1)")
        .bind(&cohost_id)
        .execute(&pool)
        .await
        .unwrap();
    let cohost_account_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO accounts (id, name, email, timezone, user_id) VALUES (?, 'Zed Co-host', 'cohost@test.com', 'UTC', ?)")
        .bind(&cohost_account_id)
        .bind(&cohost_id)
        .execute(&pool)
        .await
        .unwrap();

    let team_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO teams (id, name, slug, visibility) VALUES (?, 'Collective', 'collective', 'public')")
        .bind(&team_id)
        .execute(&pool)
        .await
        .unwrap();
    for user_id in [&organizer_id, &cohost_id] {
        sqlx::query("INSERT INTO team_members (team_id, user_id, role, source) VALUES (?, ?, 'member', 'direct')")
            .bind(&team_id)
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }
    sqlx::query("UPDATE event_types SET team_id = ?, scheduling_mode = 'collective', requires_confirmation = 1 WHERE id = ?")
        .bind(&team_id)
        .bind(&et_id)
        .execute(&pool)
        .await
        .unwrap();

    let encrypted_password = crate::crypto::encrypt_password(&[0u8; 32], "test").unwrap();
    for (account_id, calendar_href) in [
        (&organizer_account_id, "/organizer"),
        (&cohost_account_id, "/cohost"),
    ] {
        sqlx::query("INSERT INTO caldav_sources (id, account_id, name, url, username, password_enc, write_calendar_href, enabled) VALUES (?, ?, 'capture', ?, 'test', ?, ?, 1)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(account_id)
            .bind(&base_url)
            .bind(&encrypted_password)
            .bind(calendar_href)
            .execute(&pool)
            .await
            .unwrap();
    }

    let booking_id = uuid::Uuid::new_v4().to_string();
    let confirm_token = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO bookings (time_version, id, event_type_id, uid, guest_name, guest_email, guest_timezone, start_at, end_at, status, cancel_token, reschedule_token, confirm_token) VALUES (?, ?, ?, 'uid-collective-approval', 'Primary Guest', 'guest@test.com', ?, ?, ?, 'pending', ?, ?, ?)")
        .bind(time_version)
        .bind(&booking_id)
        .bind(&et_id)
        .bind(guest_timezone)
        .bind(start_at)
        .bind(end_at)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&confirm_token)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO booking_attendees (id, booking_id, email) VALUES (?, ?, 'added@test.com')",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(&booking_id)
    .execute(&pool)
    .await
    .unwrap();

    let guest_added = booking_additional_attendees(&pool, "uid-collective-approval").await;
    assert_eq!(guest_added, vec!["added@test.com"]);
    let collective_hosts = booking_collective_host_attendees(
        &pool,
        "uid-collective-approval",
        "test@example.com",
        "guest@test.com",
        &guest_added,
    )
    .await;
    assert_eq!(collective_hosts, vec!["cohost@test.com"]);

    let response = app
        .oneshot(post_bare(&format!("/booking/approve/{}", confirm_token)))
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let bodies = captured.lock().unwrap();
    assert_eq!(
        bodies.len(),
        2,
        "collective booking should reach both calendars"
    );
    for ics in bodies.iter() {
        // Content lines are folded at 75 octets (RFC 5545 §3.1).
        let ics = ics.replace("\r\n ", "");
        assert!(ics.contains(
            "ATTENDEE;SCHEDULE-AGENT=CLIENT;CN=Primary Guest;RSVP=TRUE:mailto:guest@test.com"
        ));
        assert!(ics.contains("ATTENDEE;SCHEDULE-AGENT=CLIENT;RSVP=TRUE:mailto:added@test.com"));
        assert!(ics.contains("ATTENDEE;SCHEDULE-AGENT=CLIENT;RSVP=TRUE:mailto:cohost@test.com"));
        assert_eq!(ics.matches("ATTENDEE;").count(), 3);
        assert!(!ics.contains("ATTENDEE;SCHEDULE-AGENT=CLIENT;RSVP=TRUE:mailto:test@example.com"));
    }
    let bodies = bodies.clone();
    server.abort();
    bodies
}

#[tokio::test]
async fn create_published_source_encrypts_and_redacts_bearer_url() {
    let (app, pool, session, _) = setup_test_app().await;
    let csrf = "test-csrf-published-source";
    let body = format!(
        "_csrf={}&provider=published_ics&provider_type=published_ics&name=Marathon&url=https%3A%2F%2Fexample.com%2Fprivate-calendar.ics&username=&password=&no_test=on",
        csrf
    );
    let response = app
        .clone()
        .oneshot(post_form("/dashboard/sources/new", &session, csrf, &body))
        .await
        .unwrap();
    assert!(response.status().is_redirection());
    assert_eq!(
        response.headers().get("location").unwrap(),
        "/dashboard/sources"
    );

    let row: (String, String, String, String, String, String) = sqlx::query_as(
        "SELECT name, url, username, password_enc, auth_type, provider_type
         FROM caldav_sources WHERE name = 'Marathon'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, "Marathon");
    assert_eq!(
        row.1,
        crate::providers::published_ics::PRIVATE_URL_SENTINEL,
        "plaintext bearer URL must not be stored in the source URL column"
    );
    assert!(row.2.is_empty());
    assert_eq!(row.4, "basic");
    assert_eq!(row.5, "published_ics");
    assert_eq!(
        crate::crypto::decrypt_password(&[0u8; 32], &row.3).unwrap(),
        "https://example.com/private-calendar.ics"
    );

    let dashboard = app
        .clone()
        .oneshot(get_authed("/dashboard/sources", &session))
        .await
        .unwrap();
    let dashboard_body = body_string(dashboard).await;
    assert!(dashboard_body.contains("Private published feed"));
    assert!(!dashboard_body.contains("private-calendar.ics"));

    let source_id: String =
        sqlx::query_scalar("SELECT id FROM caldav_sources WHERE name = 'Marathon'")
            .fetch_one(&pool)
            .await
            .unwrap();
    let edit = app
        .oneshot(get_authed(
            &format!("/dashboard/sources/{}/edit", source_id),
            &session,
        ))
        .await
        .unwrap();
    let edit_body = body_string(edit).await;
    assert!(edit_body.contains("Leave empty to keep current feed"));
    assert!(!edit_body.contains("private-calendar.ics"));
}

#[tokio::test]
async fn cascade_favicon_is_served() {
    let (app, _, _, _) = setup_test_app().await;
    let response = app.oneshot(get("/favicon.svg")).await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "image/svg+xml"
    );
    let body = body_string(response).await;
    assert!(body.contains("aria-label=\"Cascade\""));
    assert!(body.contains("#1A3A4A"));
    assert!(!body.contains('🦀'));
}

/// The favicon lives at a fixed URL whose *content* changes with the
/// branding, so it must never be served `immutable`: that pins whatever
/// artwork a visitor saw first for a year and makes a rebrand unshippable
/// to anyone who has already loaded the site. It must instead revalidate,
/// which requires an ETag to stay cheap. Router level because the bug was
/// in which cache helper the route was wired to, not in the helper itself.
#[tokio::test]
async fn favicon_revalidates_instead_of_being_immutable() {
    let (app, _, _, _) = setup_test_app().await;
    let response = app.oneshot(get("/favicon.svg")).await.unwrap();
    assert_eq!(response.status(), 200);
    let cache_control = response
        .headers()
        .get("cache-control")
        .expect("favicon carries a cache policy")
        .to_str()
        .expect("cache policy is ASCII")
        .to_string();
    assert!(
        !cache_control.contains("immutable"),
        "favicon URL is unversioned and its content changes with branding, \
         so it must not be immutable; got {cache_control:?}"
    );
    assert!(
        cache_control.contains("must-revalidate"),
        "favicon must revalidate; got {cache_control:?}"
    );
    assert!(
        response.headers().get("etag").is_some(),
        "favicon must carry an ETag so revalidation costs a 304, not a body"
    );
}

#[test]
fn cascade_templates_do_not_expose_upstream_branding() {
    const FORBIDDEN: &[&str] = &[
        " — calrs",
        ">calrs<",
        "calrs account",
        "calrs can",
        "calrs will",
        "calrs builds",
        "calrs POSTs",
        "olivierlambert/calrs/releases",
        "🦀",
    ];

    fn check_dir(path: &std::path::Path) {
        for entry in std::fs::read_dir(path).expect("template directory is readable") {
            let entry = entry.expect("template entry is readable");
            let path = entry.path();
            if path.is_dir() {
                check_dir(&path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("html") {
                let source = std::fs::read_to_string(&path).expect("template is readable");
                for phrase in FORBIDDEN {
                    assert!(
                        !source.contains(phrase),
                        "{} exposes upstream branding through {:?}",
                        path.display(),
                        phrase
                    );
                }
            }
        }
    }

    let base = include_str!("../../../templates/base.html");
    assert!(base.contains("<title>{% block title %}Cascade{% endblock %}</title>"));
    // The `?v=` token is part of the contract: it is what defeats favicon
    // caches poisoned before the rebrand. See `icon_version`.
    assert!(base.contains("href=\"/favicon.svg?v={{ icon_v }}\""));
    assert!(base.contains("href=\"/apple-touch-icon.png?v={{ icon_v }}\""));
    assert!(base.contains("href=\"/site.webmanifest?v={{ icon_v }}\""));

    let landing = include_str!("../../../index.html");
    assert!(landing.contains("<title>Cascade Calendar</title>"));
    assert!(landing.contains("https://calendar.cascade.tax/"));
    assert!(!landing.to_ascii_lowercase().contains("cal.rs"));
    assert!(!landing.to_ascii_lowercase().contains("calrs"));
    assert!(!landing.contains('🦀'));

    let book_config = include_str!("../../../docs/book.toml");
    assert!(book_config.contains("title = \"Cascade Calendar Documentation\""));

    for entry in std::fs::read_dir("docs").expect("rendered docs directory is readable") {
        let entry = entry.expect("rendered docs entry is readable");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("html") {
            let source = std::fs::read_to_string(&path).expect("rendered docs are readable");
            if !source.contains("<title>") {
                continue;
            }
            assert!(
                source.contains("Cascade Calendar Documentation"),
                "{} has stale documentation branding",
                path.display()
            );
            assert!(!source.contains("calrs Documentation"));
            assert!(source.contains("favicon-5a54c015.svg"));
            assert!(source.contains("favicon-5580ce5c.png"));
        }
    }
    assert_eq!(
        include_bytes!("../../../docs/favicon-5a54c015.svg"),
        include_bytes!("../../../assets/favicon.svg")
    );
    assert_eq!(
        include_bytes!("../../../docs/favicon-5580ce5c.png"),
        include_bytes!("../../../assets/favicon-192x192.png")
    );

    let readme = include_str!("../../../README.md");
    assert!(!readme.contains("assets/calrs.png"));
    assert!(readme.contains("assets/cascade-mark.png"));

    check_dir(std::path::Path::new("templates"));
}

#[tokio::test]
async fn cascade_utc_booking_pages_keep_twelve_hour_and_styled_errors() {
    let (app, pool, _session, et) = setup_test_app().await;
    sqlx::query(
        "UPDATE event_types SET timezone='America/Los_Angeles', min_notice_min=0 WHERE id=?",
    )
    .bind(&et)
    .execute(&pool)
    .await
    .unwrap();
    let mut day = Utc::now().date_naive() + Duration::days(8);
    while day.weekday() != chrono::Weekday::Mon {
        day += Duration::days(1);
    }
    let response = app
        .clone()
        .oneshot(post_form_unauthed(
            "/u/testuser/test-meeting/book",
            "c12",
            &format!(
                "_csrf=c12&date={day}&time=14:00&tz=America%2FNew_York&name=Twelve+Guest&email=twelve%40test.com"
            ),
        ))
        .await
        .unwrap();
    let response_body = body_string(response).await;
    let (start, cancel): (String, String) = sqlx::query_as(
        "SELECT start_at, cancel_token FROM bookings WHERE guest_email='twelve@test.com'",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or_else(|e| panic!("{e}: {response_body}"));
    assert!(start.ends_with('Z'), "new bookings store UTC: {start}");
    let body = body_string(
        app.clone()
            .oneshot(get(&format!("/booking/cancel/{cancel}")))
            .await
            .unwrap(),
    )
    .await;
    assert!(body.contains("2:00 PM"), "guest-local 12-hour time: {body}");
    assert!(!body.contains("14:00"), "no 24-hour display: {body}");

    // 2:30 AM on the US spring-forward Sunday does not exist.
    let response = app
        .oneshot(post_form_unauthed(
            "/u/testuser/test-meeting/book",
            "c12",
            "_csrf=c12&date=2027-03-14&time=02:30&tz=America%2FNew_York&name=Gap&email=gap%40test.com",
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    let body = body_string(response).await;
    // The styled error card, localized: the title names the class of
    // problem, the message the specific one.
    assert!(body.contains("<h1>Invalid booking details</h1>"), "{body}");
    assert!(body.contains("Invalid time."), "{body}");
}

// --- Duplicate an event type ---

async fn post_duplicate(pool: &SqlitePool, uri: &str, session: &str) -> axum::response::Response {
    let csrf = "test-csrf-duplicate";
    create_router(pool.clone(), std::env::temp_dir(), [0u8; 32])
        .await
        .oneshot(post_form(uri, session, csrf, &format!("_csrf={csrf}")))
        .await
        .unwrap()
}

async fn event_type_id_by(pool: &SqlitePool, team_id: Option<&str>, slug: &str) -> Option<String> {
    sqlx::query_scalar("SELECT id FROM event_types WHERE slug = ? AND team_id IS ?")
        .bind(slug)
        .bind(team_id)
        .fetch_optional(pool)
        .await
        .unwrap()
}

async fn column_names(pool: &SqlitePool, table: &str) -> Vec<String> {
    sqlx::query_scalar("SELECT name FROM pragma_table_info(?) ORDER BY cid")
        .bind(table)
        .fetch_all(pool)
        .await
        .unwrap()
}

/// Assert that `table` holds the same rows, ids aside, for both event types.
async fn assert_same_child_rows(pool: &SqlitePool, table: &str, source: &str, copy: &str) {
    let cols: Vec<String> = column_names(pool, table)
        .await
        .into_iter()
        .filter(|c| c != "id" && c != "event_type_id")
        .collect();
    let select = |id: &str| {
        format!(
            "SELECT {} FROM {table} WHERE event_type_id = '{id}'",
            cols.join(", ")
        )
    };
    let count = |id: String| {
        let sql = format!("SELECT COUNT(*) FROM {table} WHERE event_type_id = '{id}'");
        async move {
            sqlx::query_scalar::<_, i64>(&sql)
                .fetch_one(pool)
                .await
                .unwrap()
        }
    };
    let source_rows = count(source.to_string()).await;
    assert!(source_rows > 0, "fixture leaves {table} empty");
    assert_eq!(
        count(copy.to_string()).await,
        source_rows,
        "{table} row count"
    );
    for (a, b) in [(source, copy), (copy, source)] {
        let missing: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM ({} EXCEPT {})",
            select(a),
            select(b)
        ))
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(missing, 0, "{table} rows differ between source and copy");
    }
}

#[tokio::test]
async fn every_event_type_child_table_is_either_duplicated_or_left_behind() {
    let pool = setup_test_db().await;
    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT m.name FROM sqlite_master m
         WHERE m.type = 'table'
           AND EXISTS (SELECT 1 FROM pragma_table_info(m.name) WHERE name = 'event_type_id')
         ORDER BY m.name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for table in &tables {
        assert!(
            DUPLICATED_CHILD_TABLES.contains(&table.as_str())
                || NOT_DUPLICATED_TABLES.contains(&table.as_str()),
            "{table} references event types: decide whether Duplicate copies it"
        );
    }
    for listed in DUPLICATED_CHILD_TABLES.iter().chain(NOT_DUPLICATED_TABLES) {
        assert!(
            tables.iter().any(|t| t == listed),
            "{listed} no longer exists"
        );
    }
}

#[tokio::test]
async fn duplicating_a_team_event_type_copies_every_setting_and_child_row() {
    let (_app, pool, _admin_session, _) = setup_test_app().await;
    let (team_id, source_id) = seed_team_event_type(&pool).await;
    let (editor_id, editor_session) =
        insert_test_user_with_session(&pool, "Editor", "editor@example.com", "editor").await;
    let (member_id, member_session) =
        insert_test_user_with_session(&pool, "Member", "member@example.com", "member").await;
    for (uid, role) in [(&editor_id, "admin"), (&member_id, "member")] {
        sqlx::query(
            "INSERT INTO team_members (team_id, user_id, role, source) VALUES (?, ?, ?, 'direct')",
        )
        .bind(&team_id)
        .bind(uid)
        .bind(role)
        .execute(&pool)
        .await
        .unwrap();
    }

    // Settings on the row itself.
    sqlx::query(
        "UPDATE event_types SET description = 'Agenda', duration_min = 45, buffer_before = 5,
             buffer_after = 10, min_notice_min = 120, requires_confirmation = 1,
             location_type = 'custom', location_value = 'Teams', reminder_minutes = 1440,
             max_additional_guests = 3, visibility = 'internal', scheduling_mode = 'collective',
             default_calendar_view = 'week', first_slot_only = 1, slot_interval_min = 15,
             timezone = 'America/Los_Angeles', cancel_notice_min = 240,
             reschedule_notice_min = 480, meeting_pattern_override = 'x-{id}',
             resource_scheduling_mode = 'any', sms_phone_mode = 'required',
             booking_horizon_days = 30, pending_blocks_slot = 0, pending_expiry_hours = 48
         WHERE id = ?",
    )
    .bind(&source_id)
    .execute(&pool)
    .await
    .unwrap();

    // One row in every settings table.
    let account_id: String = sqlx::query_scalar("SELECT account_id FROM event_types WHERE id = ?")
        .bind(&source_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let calendar_id = seed_synced_event(
        &pool,
        &account_id,
        "20300107T100000",
        "20300107T110000",
        "UTC",
        "OPAQUE",
    )
    .await;
    sqlx::query("INSERT INTO event_type_calendars (event_type_id, calendar_id) VALUES (?, ?)")
        .bind(&source_id)
        .bind(&calendar_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO availability_overrides (id, event_type_id, date, start_time, end_time, is_blocked) VALUES (?, ?, '2030-01-08', '10:00', '12:00', 0)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&source_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO event_type_member_weights (event_type_id, user_id, weight) VALUES (?, ?, 7)",
    )
    .bind(&source_id)
    .bind(&editor_id)
    .execute(&pool)
    .await
    .unwrap();
    let (watcher_team, _) = insert_team_with_et(&pool, "watchers", "w", &[]).await;
    sqlx::query("INSERT INTO event_type_watchers (event_type_id, team_id) VALUES (?, ?)")
        .bind(&source_id)
        .bind(&watcher_team)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO booking_frequency_limits (id, event_type_id, max_bookings, period, per_member) VALUES (?, ?, 2, 'week', 1)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&source_id)
        .execute(&pool)
        .await
        .unwrap();
    attach_resource(&pool, &source_id, &insert_resource(&pool, "Room").await).await;

    // Guest records stay with the original.
    insert_booking(&pool, &source_id, None, dt(2030, 1, 7, 10, 0), "confirmed").await;
    sqlx::query("INSERT INTO booking_invites (id, event_type_id, token, guest_name, guest_email, created_by_user_id) VALUES (?, ?, ?, 'G', 'g@e.com', ?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&source_id)
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&editor_id)
        .execute(&pool)
        .await
        .unwrap();

    let uri = format!("/dashboard/group-event-types/{team_id}/team-sync/duplicate");

    // A plain team member cannot edit the event type, so cannot copy it.
    let response = post_duplicate(&pool, &uri, &member_session).await;
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM event_types WHERE team_id = ?")
        .bind(&team_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "a refused duplicate creates nothing");

    // A team admin can, and lands on the copy's edit page.
    let response = post_duplicate(&pool, &uri, &editor_session).await;
    assert!(response.status().is_redirection(), "{}", response.status());
    assert_eq!(
        response.headers()["location"],
        format!("/dashboard/group-event-types/{team_id}/team-sync-copy/edit").as_str()
    );
    let copy_id = event_type_id_by(&pool, Some(&team_id), "team-sync-copy")
        .await
        .expect("copy created");

    let (title, enabled, created_by): (String, i32, Option<String>) =
        sqlx::query_as("SELECT title, enabled, created_by_user_id FROM event_types WHERE id = ?")
            .bind(&copy_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(title, "Team Sync (copy)");
    assert_eq!(enabled, 0, "the copy starts disabled");
    assert_eq!(created_by.as_deref(), Some(editor_id.as_str()));

    for column in column_names(&pool, "event_types").await {
        if [
            "id",
            "slug",
            "title",
            "enabled",
            "created_at",
            "created_by_user_id",
        ]
        .contains(&column.as_str())
        {
            continue;
        }
        let same: bool = sqlx::query_scalar(&format!(
            "SELECT a.\"{column}\" IS b.\"{column}\" FROM event_types a, event_types b
             WHERE a.id = ? AND b.id = ?"
        ))
        .bind(&source_id)
        .bind(&copy_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(same, "event_types.{column} was not copied");
    }

    for table in DUPLICATED_CHILD_TABLES {
        assert_same_child_rows(&pool, table, &source_id, &copy_id).await;
    }
    for table in NOT_DUPLICATED_TABLES {
        let copied: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM {table} WHERE event_type_id = ?"
        ))
        .bind(&copy_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(copied, 0, "{table} must not be copied");
    }
}

#[tokio::test]
async fn duplicating_a_personal_event_type_picks_a_free_slug_and_refuses_others() {
    let (_app, pool, session, source_id) = setup_test_app().await;
    let uri = "/dashboard/event-types/test-meeting/duplicate";

    let response = post_duplicate(&pool, uri, &session).await;
    assert_eq!(
        response.headers()["location"],
        "/dashboard/event-types/test-meeting-copy/edit"
    );
    let response = post_duplicate(&pool, uri, &session).await;
    assert_eq!(
        response.headers()["location"],
        "/dashboard/event-types/test-meeting-copy-2/edit"
    );
    let edit = create_router(pool.clone(), std::env::temp_dir(), [0u8; 32])
        .await
        .oneshot(get_authed(
            "/dashboard/event-types/test-meeting-copy-2/edit",
            &session,
        ))
        .await
        .unwrap();
    assert_eq!(edit.status(), axum::http::StatusCode::OK);
    assert!(body_string(edit).await.contains("Test Meeting (copy)"));
    let copy_id = event_type_id_by(&pool, None, "test-meeting-copy-2")
        .await
        .unwrap();
    assert_same_child_rows(&pool, "availability_rules", &source_id, &copy_id).await;

    // Another user's personal event type is not theirs to edit or copy.
    let (_, other_session) =
        insert_test_user_with_session(&pool, "Other", "other@example.com", "other").await;
    let response = post_duplicate(&pool, uri, &other_session).await;
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);

    // Without the CSRF token nothing happens.
    let response = create_router(pool.clone(), std::env::temp_dir(), [0u8; 32])
        .await
        .oneshot(post_form(
            uri,
            &session,
            "cookie-token",
            "_csrf=other-token",
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM event_types WHERE team_id IS NULL")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 3, "the original and exactly two copies");
}

#[tokio::test]
async fn event_type_list_offers_duplicate_in_the_row_menu() {
    let (app, _pool, session, _) = setup_test_app().await;
    let body = body_string(
        app.oneshot(get_authed("/dashboard/event-types", &session))
            .await
            .unwrap(),
    )
    .await;
    let body = body.replace("&#x2f;", "/");
    assert!(body.contains(r#"action="/dashboard/event-types/test-meeting/duplicate""#));
    assert!(body.contains(">Duplicate</button>"));
}
