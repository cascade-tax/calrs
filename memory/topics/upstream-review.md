# Cal.rs upstream review and production baseline

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
