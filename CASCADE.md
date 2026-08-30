# Cascade-maintained fork

This public fork is the source for Cascade's Cal.rs deployment. Its default
branch is based on the production release pinned by
`cascade-tax/cascade-calendar`, not on unreleased upstream `main`.

## Customizations

- Human-facing web and email times always render in 12-hour format; the public
  booking picker’s 24-hour selector is intentionally removed. Stored values,
  form submissions, calendar payloads, and timezone math remain 24-hour.
- Month and week views, plus weekly booking-limit windows, start on Sunday.
- The custom theme recognizes Cascade's application palette and uses the exact
  light and dark design-system colors.
- Browser titles, dashboard branding, generated embed snippets, calendar test
  events, and notification emails use Cascade naming. The bundled favicon and
  sidebar mark are exact copies of the Cascade marketing-site assets. Internal
  compatibility identifiers retain their upstream names.
- Microsoft 365 connections request delegated read-only calendar access and
  cannot be selected as booking write-back targets.
- Cascade's Microsoft Graph schema extension is migration `064`, after
  upstream's SMS and booking-horizon migrations (`062` and `063`).

The deployment repository's weekly upstream review compares this branch with
`olivierlambert/calrs`, reviews new releases and security changes, and reports
whether these commits apply cleanly before any production update.

## Cascade image builds

`cloudbuild.yaml` stores BuildKit's complete multi-stage cache in Artifact
Registry. The cache preserves compiled Rust dependencies and intermediate
builder layers across Cloud Build workers. Build release images with:

```bash
gcloud builds submit . \
  --project cascade-calendar-prod \
  --config cloudbuild.yaml \
  --substitutions _TAG=1.17.0-cascade.N
```

The `build-cache` tag is build infrastructure, not a deployable release. Pin
production to the immutable digest of the versioned image tag.
