# Cal.rs upstream review and production baseline

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
