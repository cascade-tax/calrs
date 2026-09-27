# Cal.rs upstream review — September 27, 2026

## Decision

A reviewed update to **1.18.0-cascade.1** is warranted for the UTC booking-time correction. This is a proposed Cascade version, not an existing built artifact. Production stays at 1.17.1-cascade.1. No code port, image build, production mutation, or production authorization occurred in this review. A human approval bound to the tested image and source revision must precede deployment; do not reuse the old 1.17.1 approval.

## Sources and version comparison

- [Latest stable release, v1.18.0](https://github.com/olivierlambert/calrs/releases/tag/v1.18.0), published September 23; the release API and fetched tag agree. This is the sole intervening stable release after v1.17.1.
- [Reviewed upstream tree](https://github.com/olivierlambert/calrs/tree/fd9c2593502b4288934ddcab55279e4134b169f6) and its CHANGELOG, migration, booking-time module, OAuth change, and Google Meet implementation.
- Fork default `cascade-main` at `2180f69c3308300b1aa15687bb090d30bfbf0707`; upstream default `main` at `fd9c2593502b4288934ddcab55279e4134b169f6`. Raw ancestry: 43 fork-only and 29 upstream-only commits. The upstream revision has not moved since the September 25 review.
- Deployment checkout `cascade-tax/cascade-calendar` was fetched and fast-forward checked; `versions.env`, README, compose configuration, and the fork's CASCADE.md were read before upstream review.

## Artifact and source verification

The pinned tag `1.17.1-cascade.1` and the pinned digest both resolve in the private `us-west1` Artifact Registry repository in project `cascade-calendar-prod` to `sha256:b26907b66609f11bb0ceeb1dafd4a244f191d40b030037b4b7835af70ab5ab19`.

Cloud Build `207b20c1-76ff-4c45-acdb-6abb8f18f2ca` is SUCCESS, with the expected image/repository/location and version-tag substitutions. Its generation-pinned source archive was compared byte-for-byte with `CALRS_SOURCE_REVISION=d7f9e1a9f4a213825341b199613972ea92d9fd38`: all 285 packaged tracked files match, no differences; `.gitignore` is the one tracked file omitted from the build archive. The registry reports SLSA level unknown, so this is build-record/archive verification, not a signed source attestation.

Current fork runtime source, templates, assets, migrations, Cargo files, Dockerfile, Cloud Build configuration and publication workflow are unchanged from that built revision. Later changes are documentation and a CI pull-request branch trigger. The later fork HEAD must not be mislabeled as the deployed source.

## Release behavior, migration, and security review

1. New bookings use UTC timestamps; legacy records retain their old interpretation and are not repaired. The change reaches creation, rescheduling, cancellation, reminders, dashboard, email/ICS, availability and frequency limits. DST gaps and ambiguous starts are rejected; busy intervals cover backward clock changes. Query failures no longer silently bypass timezone/frequency checks.
2. Upstream adds `064_booking_time_version`, a version marker and UTC-format enforcement triggers. Cascade already registers `064_microsoft_graph` and adopts its legacy `062_microsoft_graph` name. Tracking uses full names, so equal numeric prefixes are not by themselves proof of runtime failure; the registry has a real merge conflict and needs an explicit, tested ordering/naming decision. Smallest port: preserve the shipped Microsoft migration identity/adoption and register the new booking-time migration at an unused Cascade number (for example 065), consistently updating SQL inclusion and tests. Do not replay Microsoft ALTER TABLE statements. Preserve upstream's new transactional migration application.
3. After any new UTC booking, binary-only rollback is unsafe. Restore the pre-upgrade database together with the old image. Bookings accepted after that backup require reconciliation if rollback occurs.
4. Google Meet links are generated with existing Google OAuth scopes, carried through email/ICS, and require eligible team members' connected write-back calendars. Meet reschedules retry the Calendar API time update three times and email the host on final failure. Approval uses the assigned host for round-robin meetings. This does not fix general CalDAV write failure reporting or collective organizer drift.
5. Security-relevant reliability changes: Google token requests have a ten-second timeout; timezone/frequency queries fail closed; migrations become transactional. Cargo.lock changes only the application version, not dependencies. The public repository security-advisory API returned an empty list; that is not a vulnerability scan or a claim of no vulnerabilities. Local TOTP MFA PR #214 remains open and is not in the stable release.
6. Other changes: personal-booking trailing-slash redirect, settings confirmation in the newly selected language, preservation of submitted settings on errors and avatar preview fixes, Google Meet translations, removal of obsolete Weblate documentation. No upstream configuration change is required. Google Meet introduces a Calendar API dependency in addition to existing CalDAV access; verify API enablement before enabling that location.

## Defect status checked live

| Item | Current status and implication |
|---|---|
| [#121](https://github.com/olivierlambert/calrs/issues/121), [PR #143](https://github.com/olivierlambert/calrs/pull/143) | Closed / merged; Google OAuth sources reconnect instead of entering the basic-auth editor, and reconnection updates rather than duplicates the source. Already present in the pinned baseline. |
| [#161](https://github.com/olivierlambert/calrs/issues/161) | Open; changing a collective team's roster can change the recomputed organizer and cause Google CalDAV to reject subsequent updates. Cascade's attendee preservation fix does not persist the organizer. |
| [PR #182](https://github.com/olivierlambert/calrs/pull/182) | Merged September 10, included in 1.18.0; Google Meet feature and its follow-up timeout/reschedule fixes are not in production. |
| [#162](https://github.com/olivierlambert/calrs/issues/162) | Open; a guest can receive confirmation despite calendar write-back failure, and failed sync can leave stale availability. |
| [#194](https://github.com/olivierlambert/calrs/issues/194) | Open; Google CalDAV versus Calendar API setup documentation remains disputed. Meet uses Calendar API; existing sync uses CalDAV. |
| [#212 / PR #215](https://github.com/olivierlambert/calrs/pull/215) | Closed / merged in 1.18.0; timezone/reminder/cancellation correction is the material upgrade reason. |
| [#208](https://github.com/olivierlambert/calrs/issues/208) | Open; a rejected settings save can still persist a username. Pre-existing, not repaired by the settings UX fixes. |
| [#209](https://github.com/olivierlambert/calrs/issues/209) | Open; language attribute and email localization complaints. |

All current open issues/PRs and the 100 most recently updated issue/PR records were inspected for relevant reports. No newer reported Google/OAuth or collective regression was found beyond those above; this does not establish absence of undiscovered regressions. Open MFA #214 and sender-name/Reply-To #222 are not stable-release changes. EWS #179 remains open and is distinct from Cascade's Graph provider.

## Cascade compatibility and proof

- Focused existing Rust tests passed: Sunday boundaries (3), Cascade palette/branding (6), time formatting (4), preserved 24-hour storage/form extraction (4), legacy Microsoft migration adoption (1). Collective attendee/availability tests (4), Microsoft integration tests (9), and published-source tests (7) also passed: 38 focused tests in total.
- Live public profile returned HTTP 200. Ten exact light/dark tokens matched: light background `#F5F3F0`, surface `#FFFFFF`, text `#22201D`, ocean `#1A3A4A`, coral `#E07A5F`; dark background `#0F1A20`, surface `#1C2A34`, text `#E8ECF0`, accent `#6FBBD1`, coral `#E88C73`. The live-served JavaScript formatter passed midnight, morning, noon, afternoon and late-night examples.
- Source inspection confirms both week-view date generators start on Sunday, month grids use Sunday offsets, and the public time toggle remains absent. Storage and submitted values remain 24-hour. This review did not repeat full interactive screenshots or authenticated booking operations.
- Cloud Build still uses BuildKit registry cache with all stages (`mode=max`), pushes a version tag to the correct private registry, and has a successful build for the pinned artifact. GitHub CI and publication workflows are active and retain fork/default-branch and manual triggers. Latest recorded GHCR publication succeeded August 18. No fresh hosted build was triggered; current-head publication is structurally checked, not newly exercised.
- A non-checkout `git merge-tree` trial against v1.18.0 found nine conflicts: archived Claude reference, booking-flow docs, CalDAV docs, `src/db.rs`, `src/email.rs`, `src/web/mod.rs`, and book/confirmed/event-type templates. No branch or worktree was created. A clean future merge is not established.

## Complete fork-only commit disposition

All 43 commits from `upstream/main..cascade-main` are accounted for below. “Port” means preserve the change's current behavior in a reviewed integration; it does not imply a conflict-free cherry-pick. Formatting-only commits can be folded into their owning feature. `git cherry` reports no exact patch-equivalent fork commit; the Weblate backport is semantically absorbed only in part because it also changes Cascade-maintained instructions.

| Commit | Change | Disposition |
|---|---|---|
| `6f2517e` | feat(cascade): enforce branded calendar presentation | Port 12-hour presentation, Sunday boundaries and exact palettes; reconcile changed booking paths. |
| `2b6c31e` | ci: allow manual fork workflows | Retain fork/manual CI and image-publish triggers. |
| `1bc1cc9` | ci: persist container build cache | Retain private Cloud Build registry-cache workflow. |
| `b37042d` | fix: initialize shared calendar scripts before page content | Port shared-script initialization order. |
| `7761706` | feat: default to Google and remove public attribution | Retain Google default and attribution removal. |
| `002ae41` | feat: complete Cascade branding | Port Cascade names, assets, notifications and documentation branding. |
| `f16168a` | tools: add reusable screenshot harness for visual UI review | Retain existing local visual verification harness. |
| `bb21ef8` | feat(ui): finish the Cascade design pass across every surface | Port Cascade design tokens and all template/email styling. |
| `e3abc87` | fix(web): styled error pages with truthful HTTP status codes | Port styled errors with truthful HTTP statuses. |
| `14e9f81` | fix(web): stop pinning branding assets in browser caches for a year | Retain branding-asset revalidation and refreshed images. |
| `3f37f94` | docs: drop the pre-rebrand screenshots | Retain removal of obsolete upstream screenshots. |
| `dd845a1` | feat: add Microsoft 365 calendar integration | Port Microsoft Graph provider; explicitly reconcile migration sequence. |
| `af5990a` | fix: make Microsoft calendar access read-only | Retain delegated read-only scope and write-back prohibition. |
| `1de4ac6` | style: format Microsoft read-only guard | Fold formatting into retained Microsoft guard. |
| `b642036` | feat: add private published calendar sources | Port private published-calendar sources and URL protection. |
| `c59e7b9` | fix: keep published source validation on Rust 2021 | Fold Rust 2021 compatibility into published-source port. |
| `2645b79` | fix: validate published calendar persistence | Port published-source validation, persistence and regression coverage. |
| `2582e6b` | fix: store individual provider events | Retain individual provider-event storage. |
| `0e57ff4` | style: format provider storage regression test | Fold formatting into provider storage port. |
| `197564e` | feat: limit published feeds to future availability | Retain future-only feed availability filtering. |
| `c73285b` | style: format future feed filtering | Fold formatting into future feed filter. |
| `ccd9d98` | feat: troubleshoot collective team availability | Port collective availability troubleshooting. |
| `46e3dc2` | fix: scope team conflict calendars per member | Port per-member conflict-calendar scope. |
| `34360e3` | chore: ignore Discord media artifacts | Retain local artifact ignore rule. |
| `0e011dc` | claude-audit: streamline repository guidance | Retain maintained guidance/review history; no executable patch to replay. |
| `9c07eba` | fix: preserve attendees on collective approval | Port collective guest/cohost attendee preservation; combine with UTC email endpoints. |
| `856eae8` | memory: record 2026-08-30 upstream review | Retain maintained guidance/review history; no executable patch to replay. |
| `10217d3` | fix: integrate Cascade customizations with v1.17.0 | Reconcile earlier release integration; retain only Cascade localization/UI deltas. |
| `0757a6a` | fix: adopt legacy Microsoft migration record | Port legacy Microsoft migration adoption without replaying ALTERs. |
| `500eefc` | memory: record Cal.rs 1.17 production deployment | Retain maintained guidance/review history; no executable patch to replay. |
| `cb00da6` | fix: integrate localized errors with status pages | Port localized errors through Cascade styled status pages. |
| `d7f9e1a` | docs: update Cascade build tag for 1.17.1 | Replace build-tag example when the new artifact is actually prepared. |
| `1ed7169` | docs: drop Weblate, document translating in the repo | Drop duplicated upstream README translation changes; retain Cascade MAINTAINERS adaptation. |
| `28cfbde` | memory: record final Cal.rs 1.17.1 baseline | Retain maintained guidance/review history; no executable patch to replay. |
| `9db1d5a` | Audit CLAUDE.md: drop the stale pre-commit hook claim | Retain maintained guidance/review history; no executable patch to replay. |
| `0e89869` | Drop the stale pre-commit hook claim from MAINTAINERS.md | Retain maintained guidance/review history; no executable patch to replay. |
| `32d90d4` | memory: record Sep 6 upstream review at 70f25ac0 | Retain maintained guidance/review history; no executable patch to replay. |
| `97ea7f9` | memory: save September 12 archive evidence corrections | Retain maintained guidance/review history; no executable patch to replay. |
| `41e1f05` | Audit CLAUDE.md: generalize the upstream-review pointer | Retain maintained guidance/review history; no executable patch to replay. |
| `8441aa4` | memory: record September 20 instruction audit | Retain maintained guidance/review history; no executable patch to replay. |
| `86c051b` | CLA-60: rename CLAUDE.md to AGENTS.md | Retain AGENTS naming and i18n CI trigger. |
| `d57852a` | Document Cal.rs 1.18 upstream review and migration conflict | Retain maintained guidance/review history; no executable patch to replay. |
| `2180f69` | memory: record September 27 instruction audit | Retain maintained guidance/review history; no executable patch to replay. |

## Smallest verification and deployment sequence

1. Port only stable 1.18.0 into an isolated candidate, preserving the inventory above. Resolve the nine conflicts and the migration registration, legacy adoption, counts and ordering. Preserve 12-hour output after UTC-to-local conversion, Sunday weekly limits, collective attendees, Graph read-only rules and private feed protections.
2. Run the existing Rust suite and upstream timezone regression script against a disposable database; cover legacy and new rows, DST, non-UTC server/event/guest zones, reminders, cancel/reschedule, approval and ICS. Rehearse migration idempotency and restoring a pre-upgrade database with the old binary. Use synthetic data first; never expose credential fields in backup inspection.
3. Verify Cascade light/dark and Sunday month/week views interactively; exercise Google reconnect/write-back and Meet create/reschedule/cancel, collective attendees, Microsoft read-only sync and the Teams bridge. Explicitly retain #161/#162 as unresolved risks.
4. Build one release candidate in the existing private Cloud Build pipeline. Verify the immutable digest and its exact source archive/revision. Produce the concrete production approval packet containing the candidate, test evidence, backup and rollback plan. Obtain Arthur's human approval before any production mutation.
5. After approval, take an application-consistent backup, deploy the pinned candidate, and smoke-check health, login, booking times, provider sync and meeting lifecycle. Roll back only with the matching pre-upgrade database and account for intervening bookings.

The review itself does not authorize this sequence's production step.
