# Cal.rs upstream review and production baseline

## October 18, 2026 — 1.20.0 reviewed, not ported

Reviewed `c3ac84a4ab36dd0db6fa01c4c9eb86fa38a96663` through
`5cb4fac26c1bcaf7fa447f965d848188b22c882e`, the exact head of
[upstream 1.20.0](https://github.com/olivierlambert/calrs/releases/tag/v1.20.0),
including PRs [#268](https://github.com/olivierlambert/calrs/pull/268),
[#270](https://github.com/olivierlambert/calrs/pull/270),
[#271](https://github.com/olivierlambert/calrs/pull/271) and
[#272](https://github.com/olivierlambert/calrs/pull/272).

The booking changes centralize guest input for write-back retries, Meet and
pending notices. Additional attendees receive reschedule updates and
cancellations after a booking was first confirmed. Approval distinguishes a
reschedule from a new request, sends matching email/SMS wording, and avoids a
second watcher offer. Calendar invites carry an increasing sequence, escape
backslashes and fold at 75 octets; readers unfold calendar data. Guest email
headlines now escape host-supplied text. Host-only phone/resource information
and cancellation reasons stay out of attendee email bodies.

All timezone pickers share grouped IANA choices, retain saved/detected aliases,
and preserve an unchanged stored zone. Editing an event type that follows its
owner keeps that inheritance. Slot views add a localized clock-change note.
The template change touches the fork's customized slot page: preserve forced
12-hour rendering, Sunday-first weeks and exact themes during any later port.

Migration 071 adds first-confirmation timestamps and insert/update triggers.
Its backfill recognizes confirmed rows and pending rows with prior-confirmation
evidence. An old pending reschedule without that evidence remains unmarked:
approval may resend the original attendee invitation, and decline/expiry/cancel
may omit withdrawal of an old invitation. Migration 072 adds sequence zero to
existing bookings; reschedules and confirmed cancellations advance it.
Both migrations are registered in `src/db.rs`. Cargo changes only the application
version; dependency declarations and dependency lock entries do not change.

OIDC parse diagnostics add bounded reason masking and response-shape summaries.
Source review found deliberate exceptions: short literals remain readable and
selected JOSE header values are logged. Treat this as diagnostic behavior to
review against the actual identity provider during an upgrade, not proof that
every possible personal value is redacted. The EWS orphan-cancellation helper
still only updates status/sequence and sends no cancellation; do not infer that
the new attendee notification coverage includes that path.

Disposition: retain production `1.18.0-cascade.2` (Oct 15; `82cff5e2`) and all custom work. Target 1.20.0 at the
next separately authorized upgrade, carrying forward October 11–13 constraints.
Renumber upstream migrations 065–072 after Cascade's existing 065, registering
the resulting 066–073 entries together. Rehearse the backfill and restore path
using a database backup; rollback requires restoring that backup with the old
binary. Preserve the Microsoft read-only boundary and implement its required
`event_exists`; retain explicit UTC Teams payloads, invitation titles and branding.

Evidence: reviewed the release/changelog, source inventory, booking notification
call sites, calendar serializers/readers, timezone handlers/templates, OIDC
diagnostics and added regression assertions. A disposable in-memory SQLite
check passed the eight-row backfill fixture, confirmation insert/update triggers,
nine sequence defaults and registration checks. No Rust compilation, upstream
test-suite run, browser/provider delivery, port, image build, production database
migration or deployment was performed. The September 28 deployment remains the
last verified production baseline in this record. Correction: the
October 15 `1.18.0-cascade.2` deployment is the current baseline (deployment
repository record). The October 18 weekly review found the upgrade warranted
and opened pending approval `66ea467f05dc46808e048ec7843b9db2`.

## October 13, 2026 — post-1.19.0 guest-detail fixes reviewed

Reviewed `e75b8d1cceca28a1baf57c2b32ed9481c6e75e26` through
`c3ac84a4ab36dd0db6fa01c4c9eb86fa38a96663`, the five-commit range merged
in [upstream PR #262](https://github.com/olivierlambert/calrs/pull/262)
for [issue #261](https://github.com/olivierlambert/calrs/issues/261).

Approval through the dashboard or emailed token, confirmed rescheduling,
claims and reminders now reload guest notes and collected phone numbers
from the booking. Host confirmation/reminder emails include those details.
Approval and confirmed rescheduling also retain additional attendees.
Google Meet's booking-details query carries the collected phone and host
language into the host calendar copy.

The calendar-copy boundary is explicit: the guest phone appears only in
the host's CalDAV copy, not email ICS attachments or the claimant's copy.
Shared resource calendars omit both guest notes and phone numbers.
Phone display follows the event type's current phone-collection mode;
stored numbers are hidden if collection has since been switched off.

Disposition: retain the production-aligned `1.18.0-cascade.1` custom fork.
Carry these fixes into the next separately authorized upgrade with the
October 12 module-split and migration constraints. This range changes
only ten Rust source/test files; it changes no version, dependency,
migration, template, or Cascade customization. No code was ported or
deployed, and no upstream compile/runtime test or clean-apply result is
claimed. Review covered the complete delta and its email, ICS, approval
and attendee regression tests. The added tests do not exercise live
Google Meet, claimant, resource-calendar or reminder delivery.

## October 12, 2026 — 1.19.0 reviewed, not ported

Reviewed `c7d627c2c45ada6f1521db236b0928cc6cd61a71` through
`e75b8d1cceca28a1baf57c2b32ed9481c6e75e26` (PRs #254–#259).
[Upstream 1.19.0](https://github.com/olivierlambert/calrs/releases/tag/v1.19.0)
was published October 11. The release gathers the previously reviewed
pending-slot holds, guest language/sender changes, calendar sync/write-back
tracking and stable organizers, plus the changes below.

The new delta splits `src/web/mod.rs` and `src/email.rs` into area modules
and moves their tests into matching directories. The architecture map now
names the owners. Log targets follow those module names; prefix filters
still match, but exact-target consumers need review during an upgrade.

Three new fixes affect the fork's existing behavior:

- Booking POST and guest reschedule recompute offered slots, rejecting
  blocked dates, out-of-hours times, off-grid times and violations of
  first-slot-only. Personal, legacy, team and dynamic-group routes share
  the slot-page computation. Added tests cover those routes, overrides,
  guest timezones and rescheduling.
- Demote, disable and delete atomically retain an enabled administrator.
  Disabled administrators do not satisfy the guard; CLI and web paths
  share it. Added coverage includes concurrent changes against SQLite WAL.
- Personal/team event-type create/edit validates titles, durations,
  nonnegative numeric fields and slugs on the server, preserving submitted
  values on rejection. Nine locales carry the new messages. The email
  source guard also checks every sender call, including attendee copies.

Disposition: keep the production-aligned custom fork at
`1.18.0-cascade.1`; do not force-sync it or deploy from this hygiene review.
The next separately authorized upgrade should target 1.19.0 and include
these fixes. Carry the October 11 constraints forward: renumber upstream
migrations 065–070 after Cascade's `065_booking_time_version`, implement
Microsoft `event_exists` while retaining read-only write-back, rehearse
the organizer/failure migrations, and preserve UTC Teams payloads,
Sunday-first slots and exact themes. The module split adds a porting step:
place each fork-only web/email change in its new owning module, including
forced 12-hour formatting and branding; do not lose it with the old files.

Source review covered the commit inventory, release metadata/changelog,
module map, booking validation helpers and call sites, atomic admin SQL,
event-type validation and added regression coverage. `src/db.rs` and
`migrations/` have no changes in this new range; Cargo's only delta is the
application version. No port, compile, runtime test, image build, migration
rehearsal or production approval is claimed. The September 28 production
baseline remains the last verified deployment.

## October 11, 2026 — unreleased upstream reviewed

Reviewed `89662d949ebb74b46db64edca6009d89616b6cb3` through
`c7d627c2c45ada6f1521db236b0928cc6cd61a71`, covering contributor merges and
PRs #247–#253. Upstream Cargo version remains 1.18.0; these changes appear
under Unreleased, not in a new production release. No code was ported,
compiled, deployed, or production-approved in this review. Production remains
the September 28 baseline below.

The delta adds CalDAV CDATA/entity parsing fixes, EWS UID fallback/window
splitting, authenticated booking prefill, guest sender/Reply-To identity,
source-sync and write-back failure tracking/retries, stable collective booking
organizers, captcha configuration fixes, and translations. Source review
covered the change inventory, changelog, migration registrations, provider
contract, retry path and fork constraints; it does not prove runtime
compatibility or cherry-pick cleanliness.

Next-release porting constraints:

- Renumber upstream migrations 065–070 after Cascade's existing
  `065_booking_time_version`. Register every renamed migration in `src/db.rs`.
- New `CalendarProvider::event_exists` requires a Cascade Microsoft-provider
  implementation while retaining the read-only write-back boundary.
- Keep Sunday-first slot/week calculations, forced 12-hour display, exact
  themes and explicit UTC Teams payloads.
- Rehearse the new write-back/source failure columns and organizer migration.
  Upstream migration 070 deliberately does not backfill existing bookings;
  new collective bookings freeze organizer identity and older ones retain
  roster-derived behavior.
- Retain the prior pending-hold/public-language migration review and drop the
  fork's async-trait lockfile bump once upstream absorbs that dependency.

The assistant hygiene policy records this reviewed head. Later upstream
movement is another review request, not permission to force-sync production.

## September 28, 2026 — 1.18.0 production baseline

Production now runs `1.18.0-cascade.1`, built from exact fork revision
`d4c55994eec7df187d69132398937cc1ae8bdf3c` (the port merged into `cascade-main`
by fast-forward and pushed), pinned to immutable Artifact Registry digest
`sha256:2d9e45364e0ce4fec937231d87c83153d2aeae7b797676f005fff1a723485b07`.
Cloud Build `fc8ba077-9836-4309-91c7-7f327fb4e8b6` succeeded, and its
generation-pinned source archive matched all 296 tracked files exactly (the
only differences were gcloud's documented default exclusion of `.git`/`.gitignore`
and two pre-existing empty untracked local directories with no files).

Migration rehearsal against a copy of the live database confirmed the
64→65 sequence — `064_microsoft_graph` (the legacy upstream `062_microsoft_graph`
adopted under that name, no ALTER replay) and `065_booking_time_version` — with
no `062_microsoft_graph` row, the three `microsoft_oauth2_*` columns present
exactly once, existing bookings' `start_at`/`end_at` unchanged (hash-verified),
all existing bookings staying at `time_version=0`, a clean `PRAGMA
integrity_check`, and an idempotent second run.

Production approval `288310edb3cb49fa9da2a2f4aa9f5dcd` was approved, bound to
the digest above, source `d4c55994`, and pre-deploy backup
`gs://cascade-calendar-backups-cascade-calendar-prod/cascade-calendar-20260928T035755Z.tar.gz`.
Deployed via the documented `ops/install-host.sh` restart path. Live
verification passed: migrations table at 65 with both new rows on the live
database, booking count and integrity unchanged, no errors in any container
log since restart, public slots pages render Sunday-first calendars, 12-hour
slot formatting, and the exact Cascade Ocean/Coral tokens in both light and
dark media queries, a bad URL returns the styled Cascade error page, the OIDC
and Google calendar-source callback routes are reachable, the Teams bridge
health check returns ok, and `ops/smoke-test-teams.py` run inside the live
`teams-bridge` container reported "Teams meeting creation and cleanup
succeeded."

## September 27, 2026 — weekly review

[Full review and all 43 fork-only commit dispositions](../recent/2026-09-27-upstream-review.md). Upstream remains at the reviewed 1.18.0 revision. An update is warranted for UTC booking correctness, but needs conflict resolution, migration rehearsal, artifact verification and human production approval. The pinned 1.17.1 artifact and all 285 packaged source files were reverified. Production was unchanged.

## September 25, 2026 — upstream 1.18.0 reviewed, not ported

Upstream `main` reached `fd9c2593502b4288934ddcab55279e4134b169f6`
with the 1.18.0 release. Since the prior reviewed revision `d1b458e02b`,
the release added UTC storage for new bookings, Google Meet auto-links,
settings fixes, and a personal-booking trailing-slash redirect. The UTC change
touches booking creation, rescheduling, reminders, cancellation, email/ICS,
frequency limits, and the dashboard. Existing booking timestamps remain in
their legacy form. Upstream's release notes require a pre-upgrade database
backup: after a new UTC booking is created, rolling back also requires
restoring the old database.

Upstream registers `064_booking_time_version`. Cascade already registers
`064_microsoft_graph`, including adoption of the older upstream migration
name, so the release cannot be ported as an unchanged migration sequence.
The booking-time change also needs compatibility testing against Cascade's
Microsoft read-only behavior and its 12-hour and Sunday-first presentation.
No code was ported, built, deployed, or approved during this hygiene review.
`cascade-main` and production remain on the pinned 1.17.1 line. A separate
upgrade must resolve the migration sequence, test the changed booking paths,
rehearse backup and rollback, and pass the existing production-approval gate.

## Archived follow-up saved September 12, 2026

The read-only review reported upstream `olivierlambert/calrs` main at
`d1b458e02b5dff7de231c0971e448984a6bf0f69`, compared with the September 6
watermark `70f25ac0b1cdecb7169f6d20cd507cea33e18a8d`.

The reported **12-commit range** is Google Meet auto-links, **PR #182 / issue
#45 phase 3**, including its feature branch, review fixes, and merge commit:

- A feature commit adds `src/google_meet.rs` (reported about 2,100 lines) and wiring.
- Four review/bugfix commits cover times/persistence/authentication, write-back
  href, reschedule PATCH retry with host-email fallback, and a workaround for
  sqlx's 16-column tuple limit introduced by the feature.
- OAuth hardening adds a **10-second reqwest timeout** to the Google token endpoint.
- Two i18n commits begin the next item in the visible report; the remainder is
  truncated, so the full 12-commit inventory cannot be reconstructed here.

This supersedes the old statement that PR #182 remained open **upstream**; it
is not evidence that Cascade has ported or deployed the feature. The visible
slice supplies no final compatibility verdict, new cherry-pick/compile/test
proof, new release status, deployment, or production approval. Do not transfer
the September 6 three-settings-fix validation to this Google Meet range. The
last confirmed production baseline is still `1.17.1-cascade.1`; the existing
release/production-approval gate remains. The original research was explicitly
read-only. This memory entry does not advance a review watermark in runtime
configuration or perform any code change.

## Sep 6, 2026

Upstream review watermark moved to `70f25ac0b1cdecb7169f6d20cd507cea33e18a8d`
(`upstream/main`). Since the previously reviewed `0f0a0d31`, upstream added
three unreleased post-1.17.1 fixes, all confined to `src/web/mod.rs` settings
handling: PR #205 confirms a dashboard language change in the newly saved
language (#204), and PR #207 keeps submitted values on a rejected settings
field (#206) and makes the avatar preview follow the edited name.

Not ported. `cascade-main` tracks the pinned production release, and these
commits are unreleased upstream `main`; they arrive with the next upstream
release under the normal production-approval gate. They were verified to apply
cleanly: all three cherry-pick onto `cascade-main` with no conflict, and
`cargo fmt --check` plus `cargo check --all-targets` pass with them applied.
The scratch branch used for that check was deleted.

None of the three touches a Cascade customization. Forced 12-hour display,
Sunday-first weeks, the exact light/dark palettes, branding, and the Microsoft
read-only guard live in other regions of `src/web/mod.rs` and in templates;
the fork has no changes in `settings_page`, `settings_render`, or
`settings_save`. Production remains `1.17.1-cascade.1`.

## Aug 30, 2026

Production runs `1.17.1-cascade.1`, built from exact Cascade fork revision
`d7f9e1a9f4a213825341b199613972ea92d9fd38` and pinned to immutable Artifact
Registry digest
`sha256:b26907b66609f11bb0ceeb1dafd4a244f191d40b030037b4b7835af70ab5ab19`.
Cloud Build `207b20c1-76ff-4c45-acdb-6abb8f18f2ca` succeeded, and its source
archive matched all 285 files in the reviewed checkout. The fork later advanced
to `1ed7169` for upstream translation-documentation corrections only; that
revision is not the built production source.

Upstream 1.17.1 was released during the work. It localizes booking error pages
and fixes five unstyled guest errors without database migrations or
configuration changes. The post-tag upstream change at review watermark
`0f0a0d3159928319747b8a5553bdd757124b4181` only removes stale Weblate
documentation and was ported to the fork.

The final suite reported 943 passed and 1 ignored; lint and format checks
passed; and all 68 visual captures passed. Live verification covered French
error localization, exact Cascade light/dark palettes, Sunday-first month/week
views, forced 12-hour display, OIDC and Google route reachability, service
health, migration metadata, and Teams meeting create/delete. The legacy
`062_microsoft_graph` adoption as `064_microsoft_graph` remains verified by the
production-backup rehearsal.

Exact 1.17.1 production approval `3833b88e239d4813822068618df1bedc` was
approved. The prior 1.17.0 approvals remain audit history, not the current
production baseline. The latest pre-deploy backup is
`gs://cascade-calendar-backups-cascade-calendar-prod/cascade-calendar-20260830T134757Z.tar.gz`.
Issue #121 and PR #143 are fixed. Issues #161, #162, and #194 and PR #182
remain open.

## October 12, 2026 — archived maintenance closeout receipt

The daily-hygiene final report says the **1.19.0 source review and maintenance records were committed and pushed**. It explicitly left production unchanged. This corroborates the reviewed/not-ported disposition at the top of this topic; it supplies no additional compile, runtime, image-build, migration-rehearsal or deployment proof. Fleet audit totals and Drawbridge's 79 tests belong to the hygiene report, not Cal.rs test coverage.

Source: supplied October 11–12 historical archive, saved October 12, 2026. This memory save did not repeat the operational work or verify live product, mailbox, runtime or deployment state.

## October 18, 2026 — archived daily-hygiene closeout

The hygiene closeout corroborates the **1.20.0 reviewed/not-ported** disposition above: booking-email fixes, timezone picker changes and two migrations reviewed; in-memory migration checks passed, including the old pending-booking case with no prior-confirmation evidence. Review and maintenance records were reportedly committed/pushed. **No upgrade was deployed.** This adds no Rust compile, upstream suite, provider delivery, backup-rehearsal or production-migration proof beyond the owning review's explicitly recorded evidence. Preserve `1.18.0-cascade.1` and the separate production-upgrade authorization requirement.

Source: supplied October 17–18 historical archive. This records reported work; the memory save did not repeat the audit, alter runtime settings, or verify deployment.
