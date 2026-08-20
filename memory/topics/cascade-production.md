# Cascade-branded production deployment

## Branch and operations ownership

- The Cascade-branded production variant of calrs is maintained on the long-lived `cascade-main` branch. The public `main` branch can evolve independently; verify the intended branch before changing or deploying the branded service.
- Production for `https://calendar.cascade.tax` is operated from the private `~/Dev/cascade-calendar` repository. That repository owns the digest-pinned image reference and VM deployment procedure.

## Current known production release

On August 19, 2026, Cascade release `1.15.1-cascade.13` was built from clean `cascade-main` revision `aaacddf` and pinned to immutable digest `sha256:d130baba6bb3d8c0bd65fc74a639d11cc31fd7ff23ae716688113a9669916bd0`. It includes collective team availability troubleshooting (`d2fecc3`) and member-scoped conflict-calendar selection (`8784b0a`). The VM update completed through the operations repository, and the running container, deployed settings template, public sign-in page, and protected-route redirects were verified healthy.

The preceding branded UI release was `1.15.1-cascade.5`, built from `4dae801` on August 18. That release completed the remaining legacy-template design pass and was verified on sign-in and Arthur's real booking page in light and dark themes.

The production database stores a custom palette that exactly equals the built-in Cascade palette. The branded renderer recognizes that equality and suppresses a redundant custom-theme override, so the built-in design tokens remain authoritative. Do not run a branding migration merely to remove that matching stored palette.

## SMTP delivery and sender identity

Production SMTP uses `smtp.gmail.com:587` with STARTTLS, authenticates as `arthur@cascade.tax`, and is configured to send as `Cascade Calendar <calendar@cascade.tax>`. Gmail initially rewrote the From and Reply-To identity to `arthur@cascade.tax` because `calendar@cascade.tax` was not authorized. Adding `calendar@cascade.tax` to Arthur's Gmail Send As identities corrected the actual sender, and subsequent end-to-end delivery was confirmed from `calendar@cascade.tax`.

The effective SMTP configuration is database-backed. The `smtp-password` Secret Manager secret had no active version when last checked; do not infer that the working app password is stored in Secret Manager.

## Design-pass verification and fixes

The August 2026 completion pass converted admin, profile/settings, team settings, invite management, date overrides, source write-back setup, guest confirmation, and the embed generator. Completion evidence reported 812 passing tests and desktop/phone visual checks in both themes. Review also corrected clipped slot scrolling, mobile theme-toggle collisions, tall-window centering, over-wide booking fields, native control color-scheme behavior, and admin checkbox layout.

## Unfinished error-response cleanup

The deployment review exposed many pre-existing handlers in the web layer that returned unstyled plain text with HTTP 200 for actual errors. The intended correction is a shared Cascade-styled error surface with semantic HTTP status codes (including not-found, conflict, gone, bad-request, forbidden, and internal-error cases). Work began after the UI deployment, but the archived transcript contains no proof that the sweep was committed, tested, or redeployed. Inspect `src/web/mod.rs`, current branch history, and production before treating this follow-up as complete.
