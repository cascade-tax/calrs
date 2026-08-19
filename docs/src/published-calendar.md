# Published Calendar (ICS)

A published iCalendar feed is the simplest way to add read-only availability
when OAuth or CalDAV is unavailable. It works with Outlook “Publish a
calendar” links and any public HTTPS URL that returns an iCalendar file.

## Connect a feed

1. Open **Dashboard > Calendar sources > Add source**.
2. Choose **Published calendar (ICS)** as the backend.
3. Enter a display name and paste one of:
   - the direct HTTPS `.ics` URL;
   - a `webcal://` subscription URL; or
   - Outlook's `outlook.live.com/?rru=addsubscription...` link.
4. Click **Add calendar source**.

Cascade extracts and normalizes the feed URL, validates it, downloads the
calendar, and syncs its events immediately. Booking pages refresh a stale feed
on demand after five minutes.

## Privacy and permissions

Published feeds are strictly read-only. They block busy times but can never be
selected as a booking write-back target.

The URL is a bearer credential: anyone who has it can read whatever the
publisher exposed. Cascade encrypts the full URL at rest and shows only
“Private published feed” in the dashboard. Publish free/busy-only data when
your calendar service supports that option. Regenerate the publishing link if
it is exposed outside trusted systems.

## Outlook notes

Outlook publishes Windows timezone IDs in ICS feeds. Cascade maps the common
Windows zones to their IANA equivalents before availability calculation, so
events retain the correct instant across timezones and daylight-saving
changes.

Published feeds do not accept writes. Confirmed Cascade bookings remain in
Cascade unless another writable calendar source is selected for write-back.
