# Cal.rs upstream review

## Aug 30, 2026

Production remains `1.15.1-cascade.14`, built from Cascade fork revision
`76c124db719625269c06b657e6da26eebdf7f597` and pinned to immutable Artifact
Registry digest
`sha256:9c04f391d281d62a1b934f84c9c45a8b3f2f6d7e7688b2777b560e449e09b5f1`.
No production update occurred.

The latest stable upstream release is `1.17.0` at
`da4a44e1027257be9b797aead7d7fbdbc2e9ab48`. An update is warranted, but the
Cascade work must be ported rather than mechanically rebased: upstream changes
conflict with the fork's templates and web code, and Cascade's custom migration
`062` collides with upstream migration numbering. The port must preserve and
reverify the forced 12-hour human-facing display, Sunday-first month/week
views, exact Cascade light/dark palettes, and Artifact Registry image-build
workflow before deployment.

Upstream issue #121 and PR #143 were fixed in 1.16. Issues #161, #162, and #194
remain open, and PR #182 remains open. Workflow Runtime approval
`162ef0b45c554b8e9e1d6148391ce830` is pending and covers preparation only. A
separate production approval tied to the exact reviewed fork revision and
immutable image digest remains required.
