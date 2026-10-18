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
  cannot be selected as booking write-back targets; the write-back path refuses
  read-only providers (Microsoft 365, published calendar feeds) outright. The
  provider's `event_exists` is a read-only lookup.
- Cascade's Microsoft Graph schema extension is migration `064`, after
  upstream's SMS and booking-horizon migrations (`062` and `063`). Databases
  that recorded it under the legacy name `062_microsoft_graph` are adopted
  without replaying its statements. Upstream 1.18.0's `064_booking_time_version`
  is registered here as `065_booking_time_version`, and upstream 1.20.0's
  `065_pending_slot_hold` through `072_booking_ics_sequence` are shifted by one
  to `066_pending_slot_hold` through `073_booking_ics_sequence`. A database at
  the 1.18.0-cascade baseline (001–065) applies exactly those eight on upgrade.
- Calendar invitation titles are the event name only (for example `Demo`);
  guest and host names are not appended.
- Failures that upstream answers with a bare HTTP 200 text body render the
  styled Cascade error card with a truthful status (404, 409, 429, ...), in
  the page's language on guest-facing pages.
- Collective co-hosts are listed as attendees on the guest's invitation,
  confirmation and reschedule alike, without receiving the guest-style
  attendee email.
- Meeting-provider webhooks always send explicit UTC start and end times
  (`...Z`), including for bookings stored before 1.18.0; the Teams bridge in
  `cascade-tax/cascade-calendar` reads them as UTC.

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
  --substitutions _TAG=1.20.0-cascade.1
```

The `build-cache` tag is build infrastructure, not a deployable release. Pin
production to the immutable digest of the versioned image tag.
