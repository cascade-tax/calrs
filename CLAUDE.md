# Cal.rs

Rust scheduling platform with CalDAV, Microsoft 365, Google Calendar, teams,
shared resources, and a web UI. This checkout is Cascade's maintained public
fork; read [CASCADE.md](CASCADE.md) before fork, release, image, or deployment
work.

## Start Here

- [README.md](README.md) and the [mdBook](docs/src/SUMMARY.md) cover product use,
  architecture, providers, authentication, booking, resources, and deployment.
- Treat `Cargo.toml`, `migrations/`, `src/`, and `templates/` as the current
  implementation map; do not maintain duplicate inventories here.
- Maintainer-only release, migration, localization, and GitHub Pages procedures
  are in [MAINTAINERS.md](MAINTAINERS.md).

## Commands

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
mdbook build docs
```

Run the narrowest checks that prove a change. Run `cargo fmt` on modified Rust
before committing; no hook enforces it in this checkout.

## Repository Rules

- New migrations must be added under `migrations/` **and** registered in the
  migration array in `src/db.rs`; verify both paths together.
- Keep credentials encrypted and follow [security.md](docs/src/security.md)
  for authentication, captcha, SSRF, CSP, and rate-limit changes.
- Use the provider abstraction in `src/providers/` for calendar backends; read
  the matching provider documentation before changing sync or write-back.
- Preserve Cascade's 12-hour human-facing display, Sunday-first calendar, theme,
  branding, and Microsoft read-only behavior documented in `CASCADE.md`.
- Translation changes follow the long-lived `i18n` branch workflow in
  `MAINTAINERS.md`; do not add untranslated inline strings to localized
  templates.

## Durable memory

- [Upstream review status](memory/topics/upstream-review.md) records reviewed
  release gaps, porting constraints, defect status, the confirmed production
  baseline, and approval gates. Read it before porting upstream work or
  changing release status.
- [Instruction audit history](memory/topics/instruction-audits.md) records dated guidance-review outcomes and no-op checkpoints.
