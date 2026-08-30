# Cal.rs upstream review

## Aug 30, 2026

Production was updated to `1.17.0-cascade.1`, built from Cascade fork revision
`8973a0faf142e8281dc4bb1d1a88368953dc3c39` and pinned to immutable Artifact
Registry digest
`sha256:f8065fb52361f12e3e4689f0b7167eb5a4d768529bdbbc40d3e4b6dbbc122da9`.
Cloud Build `3bcc1fff-8cb2-4e44-8bd1-daf91d970572` succeeded, and its archived
source matched all 285 files in the reviewed checkout.

The deployed fork is based on the latest stable upstream release, `1.17.0` at
`da4a44e1027257be9b797aead7d7fbdbc2e9ab48`. Cascade's old
`062_microsoft_graph` migration was renumbered to `064_microsoft_graph`; the
migrator adopts the legacy production record instead of replaying its schema.
The upgrade was rehearsed against a fresh production backup, and the full Rust
suite, lint/format checks, 68 desktop/mobile light/dark screenshots, live
Sunday-first rendering, Cascade palettes/branding, service health, and Teams
meeting creation/cleanup all passed.

Upstream issue #121 and PR #143 were fixed in 1.16. Issues #161, #162, and #194
remain open, and PR #182 remains open. Preparation approval
`162ef0b45c554b8e9e1d6148391ce830` and exact-artifact production approval
`eeec3bbff05141f7bb3d7b95582ba71d` were approved before deployment. The
pre-deploy backup is
`gs://cascade-calendar-backups-cascade-calendar-prod/cascade-calendar-20260830T132211Z.tar.gz`.
