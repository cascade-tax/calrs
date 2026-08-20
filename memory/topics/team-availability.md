# Team availability and conflict-calendar behavior

## Per-member conflict calendars

Team event types use member-scoped conflict-calendar selections. Each team member has a Conflict calendars action for the event type that lists only that member's own calendars marked busy. Saving replaces only rows owned by that member and never queries, renders, or overwrites teammates' calendar identities.

For a given member and event type:

- One or more selected calendars form that member's allowlist of calendars that block availability.
- No selected calendars means all of that member's calendars marked busy block availability.
- A selection made by the creator or another member must never become a global allowlist for the team.
- Submitted calendar IDs are constrained server-side to busy calendars owned by the submitting user, including during event-type creation.

Availability and troubleshooting must apply this same owner-scoped rule. The implementation reuses `event_type_calendars`; ownership is derived by joining each calendar through its source and account to the member, so no schema migration was needed.

## Collective troubleshooting

Availability Troubleshooting supports personal and collective team event types. It uses event-type IDs rather than slugs so personal/team slug collisions are unambiguous. For a collective type, it matches the public booking contract: every enabled team member with effective event-type weight greater than zero participates, and every participating member is synced before evaluation.

The diagnostic includes each participating member's calendar conflicts, confirmed bookings, and personal working hours. It may identify the member responsible for a blocked interval, but must redact teammates' calendar event summaries, calendar names, and guest details. Round-robin types remain explicitly excluded until Troubleshoot has a visualization for their "any member free" semantics.

## Implementation and release evidence

- `d2fecc3` — collective team availability in Troubleshoot, including member-aware redaction and explicit round-robin exclusion.
- `8784b0a` — member-scoped team conflict-calendar settings and owner-scoped availability filtering.
- Regression coverage proved another collective member's busy event still blocks after the creator selects a calendar and that a save mutates only the current member's rows. Targeted tests, formatting, and strict Clippy checks passed.
- The changes shipped from `cascade-main` revision `aaacddf` in production image `1.15.1-cascade.13` at `https://calendar.cascade.tax` on August 19, 2026.
