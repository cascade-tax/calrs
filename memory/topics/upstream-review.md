# Cal.rs upstream review and production baseline

## October 25, 2026 — 1.20.6 ported on branch `port-1.20.4`, not deployed

Two merges on branch `port-1.20.4`, the same way `f9c60b5` merged 1.20.0:
`f3e2641b8eb8bca0824c4aa36b43d6cc50616209` merges upstream v1.20.4
(`ac34551880b40b33277f52f6b925d4380c05d38d`) into fork head `f37d45b`, and
`05dbb395ecf52657b2c8b02c8718f6b44eebcf29` merges v1.20.6
(`95017cc171ca84f4fc34a0ec557f6cc349ed65cc`). Upstream main's later claims
work (#302, after 1.20.6) is excluded. Target release `1.20.6-cascade.1`
(coordinator decision; Arthur approved the production upgrade from
`1.20.0-cascade.2`). As before, `Cargo.toml` carries the upstream version
(`1.20.6`) and the `-cascade.N` suffix lives only in the image tag. The
branch is unbuilt, unmerged into `cascade-main` and undeployed.

1.20.5–1.20.6 merged without textual conflicts: CalDAV write-back
verification falls back to PROPFIND on HEAD 400 and reads the 207 status
(#293), propstat parsing accepts any namespace prefix (#296), German
"Powered by" wording, and the Google OAuth callback escapes echoed values
(#299). `src/caldav/mod.rs` equals upstream; the provider trait,
write-back target selection and sync cancellation are unchanged, so the
read-only boundary below still holds. Cascade addition: the callback's
four remaining bare-text failures (provider error, session change, token
exchange, account email) render the styled, auto-escaping error card with
400/502 statuses; a test sends `<script>` as the error parameter and finds
it escaped on that card.

1.20.4 port details follow.

Conflicts and resolutions:

- `src/commands/sync.rs`: the fork's generic provider sync (Graph delta,
  published feeds) keeps its shape and adopts upstream #291's verified,
  source/host/write-calendar-scoped `cancel_orphaned_booking` with
  inconclusive checks retaining the cached row. The fork's delta-tombstone
  path, which still used the removed unscoped `cancel_orphaned_booking_simple`,
  now uses the same check. Read-only sources (`supports_write_back` false:
  Microsoft 365, published feeds, or an unknown type) never look up, verify
  or cancel a booking; their sweeps only update the event cache. A new test
  drives a Graph source with a stale matching write calendar through both
  the tombstone and snapshot paths: booking stays confirmed, zero lookups.
- `src/utils.rs`: `canonical_ical_tzid` removed; upstream's `normalize_tzid`
  via `tzid::resolve` and the CLDR table replace it. `src/windows_zones.rs`
  maps India Standard Time to `Asia/Kolkata` (CLDR says `Asia/Calcutta`).
  Quote removal happens in the parameter parser before mapping; the fork
  test now covers quoted Windows names and India.
- `src/web/availability.rs`: fork's per-member troubleshoot loop kept, with
  upstream's timezone-aware `extract_exdates`.
- `templates/dashboard_sources.html`: Cascade markup, copy and the read-only
  write-calendar guard kept; #281's layout ported as page CSS (wrapping
  details, boxed sync error, two-column 44px action grid and full-width
  1rem picker at 600px and below).
- `src/web/tests/booking.rs`, `src/web/tests/emails.rs`: fork's
  `far_future_monday()` kept over upstream's next-Monday loop.

Excluded: #278's `token_error_excerpt` (raw short non-200 body text in the
error log). The parse-failure line now logs status, a redacted media type
and the value-free body summary; status recording, `client_secret_post` and
the duplicate-`openid` fix are ported. A test captures the real tracing
output for a 401 body containing `client_secret=...` and asserts no body
text appears. Fork follow-through: the published-feed provider uses the
shared TZID resolver, DURATION ends and the new EXDATE signature.

Unchanged and confirmed: vendor tree and Dockerfile identical to v1.20.4
and v1.20.6;
`cloudbuild.yaml` builds the repository root and nothing ignores `vendor/`;
migrations and `src/db.rs` identical to `cascade-main`.

Verification at the 1.20.6 merge commit (RUSTFLAGS `-D warnings`):
fmt and clippy clean; `cargo test` **1306 passed, 0 failed, 1 ignored**;
template balance 44/44; `tools/shots.sh` 68/68; scratch database 73 rows,
second run applied none; binary reports 1.20.6.

Verification at the 1.20.4 merge commit (RUSTFLAGS `-D warnings`, as CI):
`cargo fmt --check` clean; `cargo clippy --profile=test` clean; `cargo test`
**1294 passed, 0 failed, 1 ignored** (`ews_smoke`, needs a live Exchange
server), including all 20 `web::tests::cascade` tests and the 12-hour,
Sunday-first and palette tests. CI's template balance check: 44 templates,
none unbalanced. `tools/shots.sh`: 68/68 captured cleanly; sources page
inspected at mobile and desktop. Scratch-database migration: 73 rows
(`064_microsoft_graph`, `066`-`073` shifted), second run applied none.
Not run: Docker or Cloud Build image build, production-backup migration
rehearsal, live Exchange/Graph/Google/OIDC checks, deployment. A later full
sync is needed to repair cached events that lacked DTEND.

## October 25, 2026 — 1.20.2–1.20.4 reviewed, not ported

Reviewed all 33 commits and the complete 22-file delta from
`95e151a5bcdaf117c9d2d2e50cc3ef68ea1c9bb2` through
`ac34551880b40b33277f52f6b925d4380c05d38d`. The latest published release is
[1.20.4](https://github.com/olivierlambert/calrs/releases/tag/v1.20.4),
published October 25 at 10:21 UTC. Sources are the fetched immutable Git
objects, release metadata and checks on that revision.

- [#284](https://github.com/olivierlambert/calrs/pull/284) replaces property
  splitting with a quote-aware parser, handles explicit `VALUE=DATE-TIME`,
  resolves Windows/vendor/display/custom VTIMEZONE names, and converts UTC
  or differently zoned recurrence exceptions into the event's timezone.
  Per-resource memoization avoids repeating custom-zone matching. The global
  match cache clears at 256 entries. Unknown zones still degrade to floating;
  fixed-offset display-name fallback supports only whole hours and cannot
  reproduce DST. VTIMEZONE matching samples the event's start year, rather
  than implementing arbitrary custom timezone rules indefinitely.
- [#282](https://github.com/olivierlambert/calrs/pull/282) derives missing
  DTEND from DURATION for calendar sources and shared feeds. Calendar days
  preserve wall time, elapsed hours cross DST, a UTC start ignores adjacent
  TZID for duration arithmetic, and all-day events without an end last one
  day. A later upgrade needs a full sync to repair already cached events.
- [#291](https://github.com/olivierlambert/calrs/pull/291) refuses incomplete
  EWS snapshots, recursively splits truncated/find-limit windows, deduplicates
  overlapping results, and errors at the minimum window instead of treating
  missing rows as deletions. UID searches cover the full sync horizon.
  Orphan cancellation checks source, host and write-calendar ownership and
  confirms absence with the provider; inconclusive checks retain the cached
  event for retry. Confirmed cancellations use the existing email path.
  Meetings with known UTC ends in the past stay confirmed; legacy wall-clock
  rows retain the previous behavior. No live Exchange behavior was tested.
- [#278](https://github.com/olivierlambert/calrs/pull/278) records token
  endpoint status, honors providers offering only `client_secret_post`, and
  removes duplicate `openid` scope. Its new `token_error_excerpt` also logs
  any short, non-JSON, non-control-character body with a known non-200 status.
  A disposable test of that exact function proved a synthetic 401 body
  containing `client_secret=SYNTHETIC_REVIEW_SENTINEL` is returned verbatim.
  **Do not port the raw refusal-body logging.** Status and redacted shape
  diagnostics can be retained; an error HTTP status does not prove a body
  contains no credentials. No real credential or production log was used.
- [#281](https://github.com/olivierlambert/calrs/pull/281) wraps source-page
  details/actions on narrow screens. Preserve Cascade branding and the
  Microsoft read-only write-calendar guard when porting this template.
  [#288](https://github.com/olivierlambert/calrs/pull/288) changes French
  email subject nouns and assertions. Two weekday-sensitive booking tests
  now select the next Monday; Cascade already carries that test correction.

The 142-entry upstream Windows mapping covers all 25 existing Cascade
entries; the sole spelling change is India from `Asia/Kolkata` to
`Asia/Calcutta`. Preserve the existing canonical spelling when reconciling
that helper. No migrations, migration registrations, provider permission
changes, dependency-version changes (apart from the application version),
Dockerfile or vendor-data changes occur in this delta. The previously
reviewed vendored IANA 2026b correction is now in release 1.20.2.

Verification: **103 disposable offline Rust tests passed** against the exact
upstream utils, timezone, Windows-map and recurrence modules with its vendored
chrono-tz, plus the synthetic refusal-logging probe. Scratch files were removed.
Upstream Format, Templates, Coverage, and Clippy + Tests were successful;
two image-build checks were still running when inspected. Diff whitespace
checks passed. No integrated Cascade build, database/provider test, browser
check, live sync or deployment ran in this review.

Disposition: preserve documented production `1.20.0-cascade.2` and every
Cascade customization. Carry the reviewed fixes into a separately authorized
upgrade, excluding raw refusal-body logging and retaining the canonical
Windows mapping, 12-hour display, Sunday-first weeks, palettes, branding,
read-only Microsoft behavior, and fork-only booking features. No source was
ported, dependency installed, service restarted or production state changed.

## October 24, 2026 — quoted TZID and tzdata fixes reviewed, not ported

Reviewed the complete four-commit upstream range after
`9bb1e86c6a345a5dbd75f7fb780c3fd59f62d062` through
`95e151a5bcdaf117c9d2d2e50cc3ef68ea1c9bb2`, covering
[PR #280](https://github.com/olivierlambert/calrs/pull/280) and
[PR #286](https://github.com/olivierlambert/calrs/pull/286).
Latest published release remains 1.20.1; these fixes are unreleased.

The ICS reader strips surrounding quotes from a TZID parameter before using
the timezone. During a later port, retain Cascade's `canonical_ical_tzid`
Windows-to-IANA mapping and apply the quote removal before that mapping;
upstream's unchanged return path does not contain this fork customization.

The timezone fix vendors chrono-tz 0.8.6 with IANA 2026b data in rearguard
form, selects it through Cargo's local patch, and copies the vendor directory
before Docker's dependency build. The crate version and API stay unchanged.
The vendor build script adds a data-change rebuild trigger; its library adds
an allowance for deprecated chrono calls. No migrations, migration registration,
application-version bump or runtime configuration changes occur in this range.

Independent comparison found all 13 non-data vendor files identical to the
locally cached crates.io 0.8.6 package except those two documented changes.
All 12 timezone files exactly match the official
[IANA 2026b archive](https://data.iana.org/time-zones/releases/tzdata2026b.tar.gz),
with the nine zone files regenerated by that archive's own rearguard script.
Rearguard avoids literal `%z` abbreviations in the older parser. The vendored
`tz/version` says 2026b; its NEWS-derived library constant reports 2026a.

Four disposable offline Rust checks using the exact upstream parsing functions
and vendored crate passed: quoted/unquoted and UTC/floating/all-day/exact-field
parsing, folded quoted TZID, Vancouver offsets across the 2026 change with a
Los Angeles control, and Dubai's numeric abbreviation. Vancouver remains -07
in November 2026 and January 2027; the stock 2024a data would fall back to -08.
This validates the isolated changes, not the integrated Cascade application.
Upstream Format, Templates, Clippy + Tests and Coverage checks were successful
when inspected. No full Rust suite, Docker build, browser/provider check,
production timezone inventory or live verification ran in this review.

Disposition: preserve production `1.20.0-cascade.2` and every Cascade
customization. Carry these fixes and the previously reviewed 1.20.1 fixes
into a separately authorized upgrade; include the timezone correction before
November 1 if Vancouver scheduling is used. Keep the vendored-data refresh
and eventual patch-removal instructions with that port. No source was ported,
dependency installed, service restarted, migration applied or deployment made.

## October 22, 2026 — 1.20.0-cascade.2 deployed (event type duplication)

Production runs `1.20.0-cascade.2`, built from `cascade-main` at
`6848f9538b1d2e04b226e5222f0b04db3dda838d` (fork-only feature on top of the
1.20.0 baseline; upstream 1.20.1 still reviewed, not ported) and pinned to
digest `sha256:50cb4fa2d22b2294bc4391eef3dae425d369b3a0e4b25cbf99d605b7201ffa4b`.
Cloud Build `47c8336e-5c40-4c80-9dcf-5848cb5c4fd5` succeeded; its source
archive matched the commit (only `.gitignore` and two empty untracked
directories differ, as before). Fork CI passed on the release commit.

The release adds **Duplicate** to each event type's dashboard menu (upstream
issue #235 has no implementation). It uses the edit routes' permissions and
CSRF, copies every `event_types` column and settings table, leaves bookings
and invite links, and starts the copy disabled with a `-copy` slug. It also
moves two guest-phone tests off weekend dates. No migration (still 73 rows).

Authorization: Arthur's Discord request of October 22, 2026 ("add a copy
event type feature so i can do this going forward"), relayed by the
coordinating session, following the August 26 fork-patch precedent. The
deploy tooling requires no Workflow Runtime approval and none was created.
Pre-deploy backup:
`gs://cascade-calendar-backups-cascade-calendar-prod/cascade-calendar-20261022T205203Z.tar.gz`
(checksums and integrity verified). Live proof is in the deployment
repository's `memory/topics/calrs-upstream-review.md`.

## October 21, 2026 — 1.20.1 reviewed, not ported

Reviewed the complete five-commit range after
`5cb4fac26c1bcaf7fa447f965d848188b22c882e` through
`9bb1e86c6a345a5dbd75f7fb780c3fd59f62d062`, including
[PR #276](https://github.com/olivierlambert/calrs/pull/276) and
[the 1.20.1 release](https://github.com/olivierlambert/calrs/releases/tag/v1.20.1).
The release metadata dates publication to October 20, 2026.

The booking form's Back to times link now preserves an invitation token,
including when embed parameters precede it. Existing public-page handlers
supply that token; this changes navigation, not invitation validation.
Reset to my default now returns the authenticated user's profile timezone
with their availability. The form selects it, adding an option for a retained
alias when necessary, and emits a change event. An empty profile schedule
still leaves the form unchanged.

The complete delta changes eight files. Cargo changes only the application
version; no dependency, migration, migration registration or configuration
changes are present. Upstream adds route tests for the timezone response and
both embedded and ordinary invitation back links. Those tests were reviewed,
not run. A disposable Node check of the actual added timezone-selection code
passed five cases: existing zone, missing alias option, unchanged zone,
missing timezone and absent selector. No Rust build, browser test, provider
check or production check was performed.

Disposition: retain the documented `1.20.0-cascade.1` production baseline and
all fork customizations. Include these two fixes in the next separately
authorized upgrade, preserving the customized booking template and event-type
form. This source review authorizes no port, build, migration or deployment.
The assistant maintenance record pins the exact reviewed revision above.

## October 18, 2026 — 1.20.0 ported and deployed

`cascade-main` merged upstream v1.20.0 at `1f8229d` and production runs
`1.20.0-cascade.1` (digest `sha256:5647c24d...2ea573`) from that revision,
approved by Arthur (approval `66ea467f05dc46808e048ec7843b9db2`). Upstream
065-072 are registered as 066-073. Dropped as absorbed: `618cd30`
(async-trait) and the duplicate part of `9c07eba`. Fork web/email work lives in
upstream's split modules; fork-only web tests are in `src/web/tests/cascade.rs`.
Upstream's refusal pages use the styled error card with real status codes.
Eight machine-written email-footer translations are unreviewed. Full proof and
live verification are in the deployment repository's
`memory/topics/calrs-upstream-review.md`. Next upstream review starts after
`5cb4fac`.


### Archived deployment verification receipt (saved October 22, 2026)

The supplied archive corroborates the completed 1.20.0 upgrade above. The
port's tests reportedly passed. Rehearsal against a production-backup copy
applied exactly eight new migrations, preserved all bookings, passed database
integrity checks, and made a second run a no-op. No pending bookings existed
in that copy, so the reviewed old-pending-reschedule backfill risk was absent
from this dataset. A fresh pre-deployment backup was verified against the
rehearsal copy before approval was bound to the exact built image.

The deployment report says all three services were healthy and production
matched the image built from the reviewed source. All nine bookings remained
unchanged after the eight migrations, with a clean database check. Live checks
preserved Sunday-first weeks, forced 12-hour display without a 24-hour option,
and the exact pre-upgrade Cascade colors. A bad link returned the styled error
page; Google and login callback routes responded; a real Teams meeting was
created and cleaned up. Service logs showed no errors. Invite titles and UTC
webhook times were also reported retained during the port.

Source: supplied historical archive, saved October 22. These are reported
results, not checks rerun by this memory job. This receipt supplements the
October 18 deployment; the later 1.20.1 review remains reviewed/not ported.

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
