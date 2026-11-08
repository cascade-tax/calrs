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
  provider's `event_exists` is a read-only lookup. Sync sweeps of read-only
  sources (vanished events, delta tombstones) only update the local event
  cache: they never look up, verify against the provider, or cancel a booking.
- Cascade's Microsoft Graph schema extension is migration `064`, after
  upstream's SMS and booking-horizon migrations (`062` and `063`). Databases
  that recorded it under the legacy name `062_microsoft_graph` are adopted
  without replaying its statements. Upstream 1.18.0's `064_booking_time_version`
  is registered here as `065_booking_time_version`, and upstream 1.20.0's
  `065_pending_slot_hold` through `072_booking_ics_sequence` are shifted by one
  to `066_pending_slot_hold` through `073_booking_ics_sequence`. A database at
  the 1.18.0-cascade baseline (001–065) applies exactly those eight on upgrade.
  Upstream 1.20.1–1.20.6 add no migrations. Cascade's own `074_booking_firm`
  follows, so the table has 74 rows and an upstream port shifts upstream's
  074 onward by one more.
- Windows timezone names in calendar data resolve through upstream's CLDR
  table (`src/windows_zones.rs`), except that India Standard Time keeps the
  canonical `Asia/Kolkata` rather than CLDR's alias `Asia/Calcutta`. Quoted
  TZID parameters are unquoted before the lookup. The vendored chrono-tz
  (IANA 2026b) and its Dockerfile copy are kept exactly as upstream ships them.
- A token endpoint reply that fails to parse during OIDC sign-in logs its
  status, media type and value-free body shape only. Upstream 1.20.3's echo of
  short non-200 body text is not ported: a refusal can quote credentials.
- Calendar invitation titles are the event name only (for example `Demo`);
  guest and host names are not appended.
- The booking form has an optional Firm field (`bookings.firm`). When the
  guest fills it, the calendar invitation, its updates and its cancellation
  are titled `Cascade <> {firm}` instead of the event name, and host emails
  show the firm.
- Host emails about a booking (new booking, approval request and reminder,
  approval confirmation, reminder) list the attendees the guest added.
- The overview's event-type count includes team event types, matching the
  Event types page.
- Failures that upstream answers with a bare HTTP 200 text body render the
  styled Cascade error card with a truthful status (404, 409, 429, ...), in
  the page's language on guest-facing pages. This includes every failure of
  the Google Calendar connect callback, whose echoed provider text the card's
  template escapes (upstream #299 escapes the bare-text version).
- Collective co-hosts are listed as attendees on the guest's invitation,
  confirmation and reschedule alike, without receiving the guest-style
  attendee email.
- Event types can be duplicated from the dashboard row menu (upstream issue
  #235 has no implementation). The copy carries every `event_types` column
  and every settings table, starts disabled with a `-copy` slug, and leaves
  bookings and invite links behind. A guard test fails when a migration adds
  an `event_type_id` table without deciding whether it is copied.
- Approving a pending booking (email link or dashboard) answers as soon as
  the booking is confirmed. The meeting link, calendar write-back, watcher
  offers, SMS and emails run afterwards as tracked background work that a
  graceful shutdown waits for, so a slow meeting webhook neither stalls the
  page nor is cut off when the host clicks again. Cancellations delete the
  calendar copies the same way. Opening the approve page starts the hosts'
  calendar refresh that the approval re-check needs. A decline link for a
  booking that is no longer pending says it was already approved, declined
  or cancelled, as the approve link does.
- Meeting-provider webhooks always send explicit UTC start and end times
  (`...Z`), including for bookings stored before 1.18.0; the Teams bridge in
  `cascade-tax/cascade-calendar` reads them as UTC.

- Upstream 1.23.1's security fixes are backported onto the 1.20.6 base: a
  new user (registration, OIDC auto-registration, `calrs user create`) always
  gets an account of its own; startup links an orphaned account only to a
  same-address user who has none; OIDC email linking refuses an account
  already linked to another identity; and the private-address check covers
  every IPv6 private range and IPv4-embedded form. An upstream port drops
  these as already present.

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
  --substitutions _TAG=1.20.6-cascade.1
```

The `build-cache` tag is build infrastructure, not a deployable release. Pin
production to the immutable digest of the versioned image tag.
