#!/usr/bin/env python3
"""
Seed a throwaway calrs instance with demo data for the screenshot harness.

This is a self-contained replacement for ./seed_screenshots.sh, which no longer
works against the current schema (see NOTES at the bottom of this file).

It:
  1. deletes the target DB,
  2. boots `calrs serve` briefly on a scratch port so migrations run,
  3. registers alice / bob / carol over HTTP (the only way to get a valid
     Argon2 password hash without a TTY),
  4. shuts the server down and writes the demo rows directly with sqlite3.

Usage:
    python3 tools/seed.py [--binary ./target/release/calrs]
                          [--data-dir /tmp/calrs-screenshots]
                          [--password password1234]
"""

import argparse
import datetime
from http.cookies import SimpleCookie
import os
import re
import signal
import socket
import sqlite3
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import uuid

CSRF_COOKIE = "__Host-calrs_csrf"


def uid():
    return str(uuid.uuid4())


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def wait_for_port(port, timeout=45):
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=1):
                return True
        except OSError:
            time.sleep(0.2)
    return False


def http(url, data=None, cookie=None):
    """Minimal request helper. Returns (status, body, set_cookie_headers)."""
    req = urllib.request.Request(url, method="POST" if data is not None else "GET")
    if cookie:
        req.add_header("Cookie", cookie)
    body = None
    if data is not None:
        body = urllib.parse.urlencode(data).encode()
        req.add_header("Content-Type", "application/x-www-form-urlencoded")
    try:
        with urllib.request.urlopen(req, body, timeout=20) as resp:
            return resp.status, resp.read().decode("utf-8", "replace"), resp.headers.get_all("Set-Cookie") or []
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode("utf-8", "replace"), e.headers.get_all("Set-Cookie") or []


def csrf_from(set_cookies):
    for raw in set_cookies:
        c = SimpleCookie()
        c.load(raw)
        if CSRF_COOKIE in c:
            return c[CSRF_COOKIE].value
    return None


def register(base, name, email, password):
    """Register a user through the web form (double-submit CSRF)."""
    status, _body, cookies = http(f"{base}/auth/register")
    token = csrf_from(cookies)
    if not token:
        raise RuntimeError(f"no {CSRF_COOKIE} cookie on GET /auth/register (status {status})")
    # The cookie is Secure + __Host-; urllib will not replay it for us, so we
    # send it back by hand. Chrome does this natively on http://127.0.0.1.
    status, body, _ = http(
        f"{base}/auth/register",
        data={"_csrf": token, "name": name, "email": email, "password": password},
        cookie=f"{CSRF_COOKIE}={token}",
    )
    if status >= 400:
        raise RuntimeError(f"register {email} -> HTTP {status}")
    err = re.search(r'class="error"[^>]*>([^<]+)<', body)
    if err:
        raise RuntimeError(f"register {email} -> {err.group(1).strip()}")


def seed_rows(db_path, password_note):
    con = sqlite3.connect(db_path)
    con.execute("PRAGMA foreign_keys = ON")
    c = con.cursor()

    today = datetime.date.today()

    def future(days):
        return (today + datetime.timedelta(days=days)).isoformat()

    def user_id(email):
        row = c.execute("SELECT id FROM users WHERE email = ?", (email,)).fetchone()
        if not row:
            raise RuntimeError(f"user {email} was not created")
        return row[0]

    def account_id(user):
        row = c.execute("SELECT id FROM accounts WHERE user_id = ?", (user,)).fetchone()
        if not row:
            raise RuntimeError(f"no account for user {user}")
        return row[0]

    alice, bob, carol = user_id("alice@example.com"), user_id("bob@example.com"), user_id("carol@example.com")
    alice_acct, bob_acct = account_id(alice), account_id(bob)

    # ── Profiles ──────────────────────────────────────────────────────
    c.execute(
        "UPDATE users SET username='alice', role='admin', title='Engineering Lead',"
        " bio='Building great products. Based in Paris.', timezone='Europe/Paris' WHERE id=?",
        (alice,),
    )
    c.execute("UPDATE users SET username='bob', title='Product Designer', timezone='Europe/Paris' WHERE id=?", (bob,))
    c.execute("UPDATE users SET username='carol', title='Sales Manager', timezone='America/New_York' WHERE id=?", (carol,))

    # ── Personal default availability (Mon-Fri) ───────────────────────
    for u in (alice, bob, carol):
        for day in range(1, 6):
            c.execute(
                "INSERT INTO user_availability_rules (id, user_id, day_of_week, start_time, end_time) VALUES (?,?,?,?,?)",
                (uid(), u, day, "09:00", "17:00"),
            )

    # ── Event types ───────────────────────────────────────────────────
    et_intro, et_deep, et_quick, et_vip, et_old, et_bob = (uid() for _ in range(6))

    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, requires_confirmation,"
        " reminder_minutes, visibility, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_intro, alice_acct, "intro", "30-Minute Intro Call",
         "A quick intro call to discuss your needs and see if we are a good fit.",
         30, 5, 5, 60, 1, "link", "https://meet.example.com/alice", 0, 60, "public", alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, requires_confirmation,"
        " reminder_minutes, max_additional_guests, visibility, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_deep, alice_acct, "deep-dive", "60-Minute Deep Dive",
         "In-depth technical discussion. Please share context in the notes.",
         60, 10, 10, 120, 1, "link", "https://meet.example.com/alice-dd", 1, 1440, 3, "public", alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, duration_min, buffer_before, buffer_after,"
        " min_notice_min, enabled, location_type, location_value, visibility, created_by_user_id)"
        " VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_quick, alice_acct, "quick-chat", "15-Minute Quick Chat", 15, 0, 5, 30, 1, "phone",
         "+33 6 12 34 56 78", "internal", alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, is_private, visibility,"
        " requires_confirmation, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_vip, alice_acct, "vip-demo", "VIP Product Demo", "Exclusive demo for selected prospects.",
         45, 10, 10, 240, 1, "link", "https://meet.example.com/vip", 1, "private", 1, alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, duration_min, enabled, created_by_user_id)"
        " VALUES (?,?,?,?,?,?,?)",
        (et_old, alice_acct, "old-meeting", "Old Meeting Type", 30, 0, alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, created_by_user_id)"
        " VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_bob, bob_acct, "design-review", "Design Review", "Review your designs and get feedback.",
         45, 5, 5, 60, 1, "link", "https://meet.example.com/bob", bob),
    )

    # ── Team + team event type ────────────────────────────────────────
    team = uid()
    c.execute(
        "INSERT INTO teams (id, name, slug, description, visibility, created_by) VALUES (?,?,?,?,?,?)",
        (team, "Product Team", "product", "Weekly product sync with the whole team.", "public", alice),
    )
    for u, role in ((alice, "admin"), (bob, "member"), (carol, "member")):
        c.execute(
            "INSERT INTO team_members (team_id, user_id, role, source) VALUES (?,?,?,'direct')",
            (team, u, role),
        )

    private_team = uid()
    c.execute(
        "INSERT INTO teams (id, name, slug, description, visibility, invite_token, created_by)"
        " VALUES (?,?,?,?,?,?,?)",
        (private_team, "Hiring Panel", "hiring-panel", "Technical interview panel — all panelists must be free.",
         "private", uid(), alice),
    )
    for u in (alice, bob, carol):
        c.execute("INSERT INTO team_members (team_id, user_id, role, source) VALUES (?,?,'member','direct')", (private_team, u))

    et_team_rr, et_team_coll = uid(), uid()
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, team_id, scheduling_mode,"
        " visibility, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_team_rr, alice_acct, "product-sync", "Product Sync", "Talk to whoever on the team is free.",
         30, 5, 5, 60, 1, "link", "https://meet.example.com/product", team, "round_robin", "public", alice),
    )
    c.execute(
        "INSERT INTO event_types (id, account_id, slug, title, description, duration_min, buffer_before,"
        " buffer_after, min_notice_min, enabled, location_type, location_value, team_id, scheduling_mode,"
        " visibility, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        (et_team_coll, alice_acct, "panel-interview", "Panel Interview",
         "Technical interview — every panelist must be free.", 60, 10, 10, 120, 1, "link",
         "https://meet.example.com/panel", private_team, "collective", "internal", alice),
    )

    # ── Weekly availability (Mon-Fri, 09:00-12:00 and 14:00-18:00) ────
    all_ets = [et_intro, et_deep, et_quick, et_vip, et_old, et_bob, et_team_rr, et_team_coll]
    for et in all_ets:
        for day in range(1, 6):
            for start, end in (("09:00", "12:00"), ("14:00", "18:00")):
                c.execute(
                    "INSERT INTO availability_rules (id, event_type_id, day_of_week, start_time, end_time)"
                    " VALUES (?,?,?,?,?)",
                    (uid(), et, day, start, end),
                )

    # ── Date overrides (so the overrides page has content) ────────────
    c.execute(
        "INSERT INTO availability_overrides (id, event_type_id, date, is_blocked) VALUES (?,?,?,1)",
        (uid(), et_intro, future(10)),
    )
    c.execute(
        "INSERT INTO availability_overrides (id, event_type_id, date, start_time, end_time, is_blocked)"
        " VALUES (?,?,?,?,?,0)",
        (uid(), et_intro, future(14), "10:00", "13:00"),
    )

    # ── Bookings ──────────────────────────────────────────────────────
    confirmed = [
        (1, "David Park", "david@startup.io", "10:00", "10:30", et_intro, "UTC"),
        (2, "Emma Wilson", "emma@design.co", "14:00", "15:00", et_deep, "Europe/Paris"),
        (3, "Frank Mueller", "frank@company.de", "11:00", "11:30", et_intro, "Europe/Berlin"),
        (5, "Grace Kim", "grace@agency.kr", "09:00", "09:15", et_quick, "Asia/Seoul"),
        (7, "Hiro Tanaka", "hiro@tech.jp", "16:00", "16:30", et_intro, "Asia/Tokyo"),
    ]
    for days, name, email, t0, t1, et, tz in confirmed:
        d = future(days)
        bid = uid()
        c.execute(
            "INSERT INTO bookings (id, event_type_id, uid, guest_name, guest_email, guest_timezone,"
            " start_at, end_at, status, cancel_token, reschedule_token) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
            (bid, et, f"{bid}@calrs", name, email, tz, f"{d}T{t0}:00", f"{d}T{t1}:00", "confirmed", uid(), uid()),
        )

    # Team booking (round-robin assignment)
    d = future(3)
    bid = uid()
    c.execute(
        "INSERT INTO bookings (id, event_type_id, uid, guest_name, guest_email, guest_timezone,"
        " start_at, end_at, status, cancel_token, reschedule_token, assigned_user_id)"
        " VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",
        (bid, et_team_rr, f"{bid}@calrs", "Lisa Wang", "lisa@partner.com", "Asia/Shanghai",
         f"{d}T10:00:00", f"{d}T10:30:00", "confirmed", uid(), uid(), bob),
    )

    for days, name, email in ((2, "Ines Garcia", "ines@consulting.es"), (4, "Jake Thompson", "jake@venture.vc")):
        d = future(days)
        bid = uid()
        c.execute(
            "INSERT INTO bookings (id, event_type_id, uid, guest_name, guest_email, guest_timezone,"
            " start_at, end_at, status, cancel_token, reschedule_token, confirm_token, notes)"
            " VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
            (bid, et_deep, f"{bid}@calrs", name, email, "Europe/Paris", f"{d}T15:00:00", f"{d}T16:00:00",
             "pending", uid(), uid(), uid(), "Would love to discuss the enterprise plan."),
        )

    # ── Invites for the private / internal event types ────────────────
    c.execute(
        "INSERT INTO booking_invites (id, event_type_id, token, guest_name, guest_email, message,"
        " max_uses, used_count, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?)",
        (uid(), et_vip, uid(), "Sam Rivera", "sam@enterprise.com",
         "Hi Sam, here is your exclusive demo link!", 1, 0, alice),
    )
    c.execute(
        "INSERT INTO booking_invites (id, event_type_id, token, guest_name, guest_email, message,"
        " max_uses, used_count, created_by_user_id) VALUES (?,?,?,?,?,?,?,?,?)",
        (uid(), et_quick, uid(), "Nora Blake", "nora@partner.io", "Grab 15 minutes whenever suits.", 5, 2, alice),
    )

    # ── CalDAV sources / calendars / cached events ────────────────────
    # last_synced is stamped "now" on purpose: sync_if_stale() only reaches out
    # to the network for sources older than 5 minutes, and these hosts are fake.
    src_nc, src_gg = uid(), uid()
    c.execute(
        "INSERT INTO caldav_sources (id, account_id, name, url, username, password_enc, enabled,"
        " last_synced, write_calendar_href) VALUES (?,?,?,?,?,?,1,datetime('now'),?)",
        (src_nc, alice_acct, "Nextcloud", "https://cloud.example.com/remote.php/dav", "alice", "0000",
         "/remote.php/dav/calendars/alice/personal/"),
    )
    c.execute(
        "INSERT INTO caldav_sources (id, account_id, name, url, username, password_enc, enabled, last_synced)"
        " VALUES (?,?,?,?,?,?,1,datetime('now'))",
        (src_gg, alice_acct, "Google Calendar",
         "https://apidata.googleusercontent.com/caldav/v2/alice@gmail.com/", "alice@gmail.com", "0000"),
    )

    cal_personal, cal_work, cal_google = uid(), uid(), uid()
    c.execute("INSERT INTO calendars (id, source_id, href, display_name, is_busy) VALUES (?,?,?,?,1)",
              (cal_personal, src_nc, "/calendars/alice/personal/", "Personal"))
    c.execute("INSERT INTO calendars (id, source_id, href, display_name, is_busy) VALUES (?,?,?,?,1)",
              (cal_work, src_nc, "/calendars/alice/work/", "Work"))
    c.execute("INSERT INTO calendars (id, source_id, href, display_name, is_busy) VALUES (?,?,?,?,0)",
              (cal_google, src_gg, "/calendars/alice@gmail.com/events/", "Google Events"))

    ical = "BEGIN:VCALENDAR\r\nEND:VCALENDAR"
    for i in range(1, 6):
        d = future(i)
        eid = uid()
        c.execute(
            "INSERT INTO events (id, calendar_id, uid, summary, start_at, end_at, raw_ical, etag, all_day, status)"
            " VALUES (?,?,?,?,?,?,?,?,0,'CONFIRMED')",
            (eid, cal_work, f"{eid}@nextcloud", "Team Standup", f"{d}T09:30:00", f"{d}T10:00:00", ical, f"etag-{i}"),
        )
    d = future(2)
    eid = uid()
    c.execute(
        "INSERT INTO events (id, calendar_id, uid, summary, start_at, end_at, raw_ical, etag, all_day, status)"
        " VALUES (?,?,?,?,?,?,?,?,0,'CONFIRMED')",
        (eid, cal_personal, f"{eid}@nextcloud", "Lunch with Sarah", f"{d}T12:00:00", f"{d}T13:30:00", ical, "etag-lunch"),
    )

    con.commit()

    summary = {
        "alice_id": alice,
        "event_type_slug": "intro",
        "event_type_id": et_intro,
        "team_slug": "product",
        "password": password_note,
    }
    con.close()
    return summary


def main():
    repo = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", default=os.path.join(repo, "target", "release", "calrs"))
    ap.add_argument("--data-dir", default="/tmp/calrs-screenshots")
    ap.add_argument("--password", default=os.environ.get("CALRS_SHOTS_PASSWORD", "password1234"))
    args = ap.parse_args()

    if len(args.password) < 12:
        sys.exit("calrs requires passwords of at least 12 characters (src/auth.rs:736)")
    if not os.path.exists(args.binary):
        sys.exit(f"binary not found: {args.binary}")

    os.makedirs(args.data_dir, exist_ok=True)
    for suffix in ("", "-wal", "-shm"):
        path = os.path.join(args.data_dir, "calrs.db" + suffix)
        if os.path.exists(path):
            os.remove(path)
    db_path = os.path.join(args.data_dir, "calrs.db")

    port = free_port()
    print(f"[seed] booting {args.binary} on 127.0.0.1:{port} to run migrations…")
    proc = subprocess.Popen(
        [args.binary, "serve", "--port", str(port), "--data-dir", args.data_dir],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        if not wait_for_port(port):
            sys.exit("[seed] server did not come up")
        base = f"http://127.0.0.1:{port}"
        for name, email in (("Alice Martin", "alice@example.com"),
                            ("Bob Chen", "bob@example.com"),
                            ("Carol Davis", "carol@example.com")):
            register(base, name, email, args.password)
            print(f"[seed] registered {email}")
    finally:
        proc.send_signal(signal.SIGTERM)
        try:
            proc.wait(timeout=15)
        except subprocess.TimeoutExpired:
            proc.kill()

    info = seed_rows(db_path, args.password)
    print("[seed] demo data written")
    for k, v in info.items():
        print(f"[seed] {k}={v}")


if __name__ == "__main__":
    main()


# NOTES — why ./seed_screenshots.sh no longer works (2026-08):
#   1. It registers with the password "password123" (11 chars). Registration now
#      requires >= 12 characters (src/auth.rs:736), so no users are created and
#      every later step fails on a missing alice.
#   2. The CSRF cookie is now "__Host-calrs_csrf" and Secure, so curl will not
#      replay it over plain HTTP; the double-submit check fails. (Chrome treats
#      127.0.0.1 as a secure origin, so a browser is unaffected.)
#   3. It inserts into team_links / team_link_members / team_link_bookings,
#      dropped by migration 035. Those inserts are in the same uncommitted
#      transaction as everything else, so the whole seed is rolled back.
