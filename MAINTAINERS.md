# Maintainers

calrs is maintained by a small group of people, with a single project lead and per-feature maintainers who own specific integrations they actively use. The lead reviews and merges all changes; provider maintainers are the first point of contact and the de-facto reviewer for changes touching their area.

## Project maintainer

- **Olivier Lambert** ([@olivierlambert](https://github.com/olivierlambert))
  Overall direction, releases, security review, code style. Final say on every merge.

## Provider maintainers

Each CalDAV provider integration is owned by someone who actually runs that provider in production. The owner verifies that PRs touching the provider keep it working, triages provider-specific bug reports, and is consulted when the project lead reviews changes that affect the integration.

Listing a maintainer here does **not** transfer commit rights, it sets the expectation of who reviews and signs off on changes for that area.

| Provider | Maintainer | Status |
|---|---|---|
| BlueMind | [@olivierlambert](https://github.com/olivierlambert) | Primary test target. |
| Nextcloud | _(seeking maintainer)_ | Used by some users; no dedicated owner. |
| Google Calendar | [@bboles](https://github.com/bboles) | OAuth2 path added in #99. |
| SOGo | _(seeking maintainer)_ | Not personally tested by the project lead. |
| Zimbra | _(seeking maintainer)_ | Not personally tested by the project lead. |
| Radicale | _(seeking maintainer)_ | Not personally tested by the project lead. |
| iCloud | _(seeking maintainer)_ | Not personally tested by the project lead. |
| Fastmail | _(seeking maintainer)_ | Not personally tested by the project lead. |

## Becoming a provider maintainer

If you actively use one of the providers above (or want to add a new one) and are willing to:

- Run a recent calrs build against your provider periodically.
- Respond to bug reports tagged with your provider in a reasonable timeframe.
- Review PRs that touch your provider's integration code.

…then open an issue or comment on an existing one and we'll add you here. You don't need to commit to anything formal beyond "I care that this keeps working."

## What happens if a provider has no maintainer

We don't promise that integrations without a dedicated maintainer keep working across releases. If a refactor breaks an unmaintained provider and no test catches it, the breakage may ship and only get fixed when someone who uses that provider files a bug. Provider maintainers exist to give those integrations a faster feedback loop.

## Development workflows

Run `cargo fmt --check`, `cargo check`, `cargo test`, and
`cargo clippy -- -D warnings` as appropriate for the changed Rust surface. The
pre-commit hook enforces canonical `rustfmt` output.

When adding a migration, create `migrations/NNN_description.sql` and register it
in the migration array inside `src/db.rs::migrate()`. Verify both paths together;
an unregistered migration does not run on existing deployments.

### Localization and Weblate

The `i18n` branch is permanent. Hosted Weblate pushes translations there; merge
`i18n` into the release branch periodically, and do not delete or recreate the
branch. Do not merge the release branch back into `i18n` merely to synchronize
it.

For new or changed translatable text:

1. Branch from `i18n` and add the English source key to
   `i18n/en/main.ftl`; missing locale keys fall back to English.
2. Use the existing Minijinja `t()` helper in localized templates and provide
   the appropriate guest or authenticated language context.
3. Push to `i18n` so Weblate can translate the key before the next release
   merge.

For a new locale, create `i18n/{code}/main.ftl`, register it in
`SUPPORTED_LANGS`, add its label to `supported_with_labels()`, and push it to
`i18n` for Weblate discovery.

### Publishing the documentation site

The landing page and mdBook output live on `gh-pages`:

1. On the release branch, run `mdbook build docs`.
2. Switch to `gh-pages`, restore `docs/src` and `docs/book.toml` from the
   release branch, and rebuild.
3. Replace the published files under `docs/` with `docs/book/`; update
   `index.html` when the landing page changed.
4. Stage only `docs/` and `index.html`, commit with `--no-verify` because the
   branch does not contain `Cargo.toml`, push, and switch back to the release
   branch.
