# Cascade-branded production deployment

## Branch and operations ownership

- The Cascade-branded production variant of calrs is maintained on the long-lived `cascade-main` branch. The public `main` branch can evolve independently; verify the intended branch before changing or deploying the branded service.
- Production for `https://calendar.cascade.tax` is operated from the private `~/Dev/cascade-calendar` repository. That repository owns the digest-pinned image reference and VM deployment procedure.

## Current known production release

On August 18, 2026, calrs commit `4dae801` completed the Cascade design pass across the remaining legacy templates and was deployed as image `1.15.1-cascade.5`. Cloud Build built the image, the operations repository pinned it by digest, and the VM service was restarted over IAP. Health checks and live visual checks covered sign-in and Arthur's real booking page in both light and dark themes.

The production database stores a custom palette that exactly equals the built-in Cascade palette. The branded renderer recognizes that equality and suppresses a redundant custom-theme override, so the built-in design tokens remain authoritative. Do not run a branding migration merely to remove that matching stored palette.

## Design-pass verification and fixes

The August 2026 completion pass converted admin, profile/settings, team settings, invite management, date overrides, source write-back setup, guest confirmation, and the embed generator. Completion evidence reported 812 passing tests and desktop/phone visual checks in both themes. Review also corrected clipped slot scrolling, mobile theme-toggle collisions, tall-window centering, over-wide booking fields, native control color-scheme behavior, and admin checkbox layout.

## Unfinished error-response cleanup

The deployment review exposed many pre-existing handlers in the web layer that returned unstyled plain text with HTTP 200 for actual errors. The intended correction is a shared Cascade-styled error surface with semantic HTTP status codes (including not-found, conflict, gone, bad-request, forbidden, and internal-error cases). Work began after the UI deployment, but the archived transcript contains no proof that the sweep was committed, tested, or redeployed. Inspect `src/web/mod.rs`, current branch history, and production before treating this follow-up as complete.
