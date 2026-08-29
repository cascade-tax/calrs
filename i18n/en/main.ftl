# Booking confirmation page (templates/confirmed.html)

confirmed-page-title-pending = Booking pending
confirmed-page-title-booked = Booking confirmed

confirmed-heading-reschedule-requested = Reschedule requested
confirmed-heading-rescheduled = Rescheduled!
confirmed-heading-pending = Pending confirmation
confirmed-heading-booked = You're booked!

confirmed-subtitle-reschedule-requested = Your reschedule request has been sent to { $host }. You'll receive an email at { $email } once it's approved.
confirmed-subtitle-rescheduled = Your booking has been rescheduled. A confirmation email has been sent to { $email }.
confirmed-subtitle-pending = Your booking request has been sent to { $host }. You'll receive an email at { $email } once it's confirmed.
confirmed-subtitle-booked = A confirmation email has been sent to { $email }.

confirmed-detail-event = Event:
confirmed-detail-date = Date:
confirmed-detail-time = Time:
confirmed-detail-with = With:
confirmed-detail-location = Location:
confirmed-detail-notes = Notes:
confirmed-detail-additional-guests = Additional guests:

confirmed-book-another = Book another time

confirmed-add-to-calendar = Add to calendar

# Slot picker (templates/slots.html)

slots-location-video = Video call
slots-location-phone = Phone call

slots-tz-label = Your timezone
slots-time-format-label = Time format

slots-view-month = Month view
slots-view-week = Week view
slots-view-column = Column view

slots-weekday-mon = Mon
slots-weekday-tue = Tue
slots-weekday-wed = Wed
slots-weekday-thu = Thu
slots-weekday-fri = Fri
slots-weekday-sat = Sat
slots-weekday-sun = Sun

slots-weekday-mon-short = M
slots-weekday-tue-short = T
slots-weekday-wed-short = W
slots-weekday-thu-short = T
slots-weekday-fri-short = F
slots-weekday-sat-short = S
slots-weekday-sun-short = S

slots-select-date = Select a date
slots-loading-availability = Loading availability...
slots-click-highlighted = Click a highlighted date to see available times
slots-no-times-month = No available times this month
slots-no-times-day = No available times this day
slots-no-availability-participants = No availability found for all participants this month
slots-week-more = more

# Booking form (templates/book.html)

book-page-title = Book { $title }
book-back-to-times = Back to times
book-name-label = Your name
book-name-placeholder = Jane Doe
book-email-label = Email
book-email-placeholder = jane@example.com
book-email-invalid = Please enter a complete email address, including the domain (e.g. jane@example.com).
book-notes-label = Notes
book-notes-optional = (optional)
book-notes-placeholder = Anything you'd like to discuss?
book-additional-guests-label = Additional guests
book-additional-guests-hint = (optional, up to { $max })
book-add-guest-btn = + Add guest email
book-guest-email-placeholder = colleague@example.com
book-phone-label = Phone number
book-phone-placeholder = 06 12 34 56 78
book-phone-help = Local numbers are fine; { $country } is assumed unless you start with +.
book-phone-optional-consequence = Leave it empty if you would rather not get text messages about this booking.
book-phone-required = A phone number is required for this booking.
book-phone-invalid-title = Invalid phone number
book-phone-invalid = Please enter a phone number we can text, or leave the field empty.
book-phone-country-search = Search
book-phone-country-label = Select country
book-phone-country-none = No country selected
book-phone-country-no-results = No countries match that search
captcha-label = Security verification
captcha-initial-state = Verify you're human
captcha-verifying = Verifying...
captcha-solved = You're human
captcha-error = Error
captcha-troubleshooting = Troubleshooting
captcha-wasm-disabled = Enable WASM for significantly faster solving
captcha-verify-aria = Click to verify you're a human
captcha-verifying-aria = Verifying, please wait
captcha-verified-aria = Verified
captcha-required = Please verify you're human
captcha-error-aria = An error occurred, please try again
book-confirm-button = Confirm booking

# SMS notifications (src/sms/message.rs).
#
# These are text messages, billed per 160-character segment (70 if the text
# contains any character outside the GSM-7 alphabet, which includes most
# accented letters). Keep them short and plain.

sms-confirmed = Booking confirmed: { $event }, { $date } at { $time } ({ $tz }).
sms-cancelled = Booking cancelled: { $event }, { $date } at { $time } ({ $tz }).
sms-rescheduled = Booking moved: { $event } is now { $date } at { $time } ({ $tz }).
sms-reminder = Reminder: { $event } starts { $date } at { $time } ({ $tz }).

# Shared labels used across the cancel / decline / approve / reschedule / claim flows

common-detail-guest = Guest:
common-detail-reason = Reason:
common-reason-optional = (optional)
common-close-page = You can close this page.

# Cancel flow (booking_cancel_form.html, booking_cancelled_guest.html)

cancel-page-title = Cancel booking
cancel-heading = Cancel booking
cancel-subtitle = You are about to cancel your booking.
cancel-reason-label = Reason
cancel-reason-placeholder-host = Let the host know why...
cancel-button = Cancel booking
cancelled-heading = Booking cancelled
cancelled-subtitle = Your booking has been cancelled and the host has been notified.

# Decline flow (booking_decline_form.html, booking_declined.html)

decline-page-title = Decline booking
decline-heading = Decline booking
decline-subtitle = You are about to decline this booking request.
decline-reason-placeholder-guest = Let the guest know why...
decline-button = Decline booking
declined-heading = Booking declined
declined-subtitle = The booking has been declined and the guest has been notified.

# Approve flow (booking_approve_form.html, booking_approved.html)

approve-page-title = Approve booking
approve-heading = Approve booking
approve-subtitle = You are about to approve this booking request.
approve-button = Approve booking
approved-heading = Booking approved
approved-subtitle = The booking has been confirmed and a confirmation email has been sent to { $email }.

# Claim flow (booking_claim_form.html, booking_claimed.html, booking_already_claimed.html)

claim-page-title = Claim booking
claim-heading = Claim booking
claim-subtitle = You are about to claim this booking. You will be added as an attendee.
claim-assigned-to = Assigned to:
claim-button = Claim this booking
claimed-page-title = Booking claimed
claimed-heading = Booking claimed
claimed-subtitle = You have claimed this booking. A calendar invite has been sent to your email.
already-claimed-page-title = Already claimed
already-claimed-heading = Already claimed
already-claimed-subtitle = This booking has already been claimed by { $name }.

# Generic error page (booking_action_error.html)

action-error-page-title = Booking action error

# Host-initiated reschedule (booking_host_reschedule.html)

host-resched-page-title = Reschedule booking — calrs
host-resched-heading = Reschedule booking
host-resched-subtitle = This will send { $guest } an email asking them to pick a new time.
host-resched-currently = Currently:
host-resched-button = Send reschedule request
host-resched-cancel-link = Cancel

# Guest reschedule confirmation (booking_reschedule_confirm.html)

resched-confirm-page-title = Confirm reschedule
resched-confirm-heading = Confirm reschedule
resched-confirm-subtitle = You are about to move your booking to a new time.
resched-was = Was:
resched-new = New:
resched-button = Confirm reschedule
resched-back-to-picker = Back to time picker

# Base layout chrome (templates/base.html)

base-loader-checking = Checking availability
base-loader-please-wait = Please wait, loading the latest calendar data...
base-stop-impersonating = Stop impersonating
base-theme-toggle = Toggle theme
base-powered-by = Powered by

# Profile (templates/profile.html)

profile-pick-event-type-invite = Pick an event type to book a time.
profile-no-event-type = No event types available yet.

# Month and weekday names + per-locale date format patterns.
# Used by server-side date formatters in src/i18n.rs.

common-month-1 = January
common-month-2 = February
common-month-3 = March
common-month-4 = April
common-month-5 = May
common-month-6 = June
common-month-7 = July
common-month-8 = August
common-month-9 = September
common-month-10 = October
common-month-11 = November
common-month-12 = December

common-weekday-long-mon = Monday
common-weekday-long-tue = Tuesday
common-weekday-long-wed = Wednesday
common-weekday-long-thu = Thursday
common-weekday-long-fri = Friday
common-weekday-long-sat = Saturday
common-weekday-long-sun = Sunday

# Format patterns are parametric per locale to handle word order. Translators
# pick where each placeholder lands. Example outputs:
#   EN: April 2026  /  Tuesday, March 12, 2026
#   FR: avril 2026  /  mardi 12 mars 2026
#   ES: abril 2026  /  martes, 12 de marzo de 2026
common-format-month-year = { $month } { $year }
common-format-long-date = { $weekday }, { $month } { $day }, { $year }

# Email signatures and shared bits (src/email.rs)

email-signature = — calrs
email-action-reschedule = Reschedule
email-action-cancel-booking = Cancel booking

# Email: guest booking confirmation

# Kept to "event — date": Exchange titles the guest appointment after the
# email Subject header, not the ICS SUMMARY (#157).
email-confirm-subject = { $event } — { $date }
email-confirm-greeting = Hi { $name },
email-confirm-headline = Your booking has been confirmed!
email-confirm-ics-attached-plain = A calendar invite is attached.
email-confirm-ics-attached-html = A calendar invite is attached to this email.
email-confirm-need-to-cancel = Need to cancel? { $url }

# Email: guest reminder

email-reminder-subject = Reminder: { $event } at { $time }
email-reminder-headline = Your meeting is coming up.

# Email: guest cancellation

email-cancel-subject = Cancelled: { $event } — { $date }
email-cancel-headline-by-host = Your booking has been cancelled by { $host }.
email-cancel-headline-by-guest = Your booking has been cancelled.
email-cancel-ics-attached-plain = A calendar cancellation is attached.
email-cancel-ics-attached-html = A calendar cancellation is attached to this email.

# Confirmation email: notice-window policy lines (src/email.rs)

email-confirm-cancel-notice = Note: cancellation requires at least { $minutes } minutes notice.
email-confirm-reschedule-notice = Note: rescheduling requires at least { $minutes } minutes notice.

# Event type form: cancel/reschedule minimum notice (templates/event_type_form.html)

event-type-form-cancel-notice-label = Minimum notice to cancel
event-type-form-reschedule-notice-label = Minimum notice to reschedule
event-type-form-notice-help = Leave empty for no restriction.
event-type-form-resources-label = Required resources
event-type-form-resources-hint = Slots are offered only when the selected resources are available, according to the mode below.
event-type-form-resources-mode-all = All selected resources must be free
event-type-form-resources-mode-round-robin = Any one free resource is enough (it gets assigned to the booking)
event-type-form-notice-unit-minutes = minutes
event-type-form-notice-unit-hours = hours
event-type-form-notice-unit-days = days
event-type-form-booking-horizon-label = Booking horizon
event-type-form-booking-horizon-help = How many days ahead guests can book. Leave empty for no limit, 0 for today only.

# Booking confirmation: cancel/reschedule policy notices (templates/confirmed.html)

confirmed-cancel-notice-info = Cancellation requires at least { $minutes } minutes notice before the meeting.
confirmed-reschedule-notice-info = Rescheduling requires at least { $minutes } minutes notice before the meeting.

# Booking action blocked page (templates/booking_action_blocked.html)

booking-blocked-title-cancel = This booking can no longer be cancelled online
booking-blocked-title-reschedule = This booking can no longer be rescheduled online
booking-blocked-body = The host requires at least { $minutes } minutes of notice. If you cannot attend, please email <a href="mailto:{ $host_email }">{ $host_email }</a> directly.

# Dashboard event types listing (templates/dashboard_event_types.html)

dashboard-event-types-copy = Copy
dashboard-event-types-copied = Copied!
dashboard-event-types-copy-title = Copy booking link
dashboard-event-types-copy-failed = Copy failed

# Dashboard sidebar and shared chrome (templates/dashboard_base.html)

nav-section-scheduling = Scheduling
nav-overview = Overview
nav-event-types = Event Types
nav-bookings = Bookings
nav-teams = Teams
nav-section-shared-links = Shared Links
nav-invite-links = Invite Links
nav-section-calendars = Calendars
nav-sources = Sources
nav-section-personal = Personal
nav-settings = Profile & Settings
nav-troubleshoot = Troubleshoot
nav-section-admin = Admin
nav-admin-panel = Admin Panel
nav-sign-out = Sign out
nav-release-notes = View release notes

# Timezone mismatch banner (templates/dashboard_base.html)

tz-banner-text = Your browser timezone is { $detected } but your booking timezone is set to { $current }.
tz-banner-update = Update
tz-banner-dismiss = Dismiss

# Markdown editor toolbar (templates/dashboard_base.html)

editor-link-prompt = Enter URL:
editor-link-default-label = link text
editor-placeholder-text = text
editor-nothing-to-preview = Nothing to preview

# Dashboard overview (templates/dashboard_overview.html)

overview-page-title = Dashboard
overview-welcome = Welcome, { $name }
overview-public-page = Public page:
overview-avail-banner-title = Default availability
overview-avail-banner-body = Your default working hours have been set to Mon–Fri, 9:00–17:00. These are used when others include you in dynamic group meetings.
overview-avail-banner-cta = Review your availability
overview-dismiss = Dismiss
overview-getting-started = Getting started
overview-getting-started-help = Follow these steps to start accepting bookings.
overview-step-connect-calendar = Connect a calendar
overview-step-first-event-type = Create your first event type
overview-step-share-link = Share your booking link
overview-pending-approval = Pending approval
overview-booking-with = { $title } with { $guest }
overview-badge-pending = pending
overview-guest-booked = Guest booked:
overview-confirm = Confirm
overview-decline = Decline
overview-stat-event-types = Event Types
overview-stat-upcoming = Upcoming Bookings
overview-stat-pending = Pending Approval
overview-stat-sources = Calendar Sources
overview-quick-actions = Create a new event type
overview-action-public-title = Public booking page
overview-action-public-desc = Share a link — anyone can pick a slot and book time with you.
overview-action-team-title = Team scheduling
overview-action-team-desc = Distribute bookings across team members or find a time everyone's free.
overview-action-team-desc-empty = Create a team first, then set up shared event types.
overview-action-private-title = Private invite-only
overview-action-private-desc = Generate single-use links for specific contacts. No one else can book.
overview-action-shared-title = Shared invite links
overview-action-shared-desc = Any colleague in the team can generate booking links to share externally.
overview-action-reason-calendar = Connect a calendar first
overview-action-reason-ask-admin = Ask an admin to create a team
overview-action-reason-team-admin = Requires a team — create one first
overview-action-reason-team-member = Requires a team — ask an admin

# Dashboard bookings (templates/dashboard_bookings.html)

bookings-page-title = Bookings
bookings-pending-approval = Pending approval
bookings-available-to-claim = Available to claim
bookings-upcoming = Upcoming bookings
bookings-with = { $title } with { $guest }
bookings-guest-booked = Guest booked:
bookings-resource = Resource:
bookings-confirm = Confirm
bookings-reschedule = Reschedule
bookings-decline = Decline
bookings-claim = Claim
bookings-badge-awaiting-reschedule = awaiting reschedule
bookings-cancel = Cancel
bookings-reason-placeholder = Reason (optional)
bookings-confirm-cancel = Confirm cancel
bookings-back = Back
bookings-empty = No upcoming bookings yet.<br>Share your { $link } so people can book time with you.
bookings-empty-link-label = event type links

# Dashboard teams listing (templates/dashboard_teams.html)

teams-page-title = Teams
teams-heading = Teams
teams-new = New
teams-badge-public = public
teams-badge-private = private
teams-settings = Settings
teams-view = View
teams-empty = No teams yet.
teams-empty-admin = { $link } to collaborate with your team.
teams-empty-admin-link-label = Create one
teams-empty-member = Teams are created by administrators. Ask your admin to create one and add you as a member.

# Dashboard invite links (templates/dashboard_internal.html)

invite-links-page-title = Invite Links
invite-links-heading = Invite Links
invite-links-new = New internal event
invite-links-help = Generate single-use booking links for internal event types. Any authenticated colleague can create and share links here.
invite-links-duration = { $minutes }min
invite-links-hosted-by = Hosted by { $host }
invite-links-get-link = Get link
invite-links-invites = Invites
invite-links-empty = No internal event types yet.<br>{ $link } with "Internal" visibility to let any colleague generate booking links.
invite-links-empty-link-label = Create an event type
invite-links-js-generating = Generating...
invite-links-js-copied = Copied!
invite-links-js-error = Error

teams-member-count =
    { $count ->
        [one] { $count } member
       *[other] { $count } members
    }

# Dashboard calendar sources (templates/dashboard_sources.html)

sources-page-title = Calendar Sources
sources-heading = Calendar sources
sources-add = Add
sources-last-sync = Last sync:
sources-sync = Sync
sources-full-resync = Full resync
sources-full-resync-title = Clear cache and re-fetch all events from server
sources-test = Test
sources-reconnect = Reconnect
sources-reconnect-title = Re-run the Google consent flow
sources-edit = Edit
sources-remove = Remove
sources-remove-confirm = Remove source '{ $name }'? This will delete all synced events from this source.
sources-no-write-calendar = No write calendar selected. Confirmed bookings stay in calrs and are not pushed to this calendar. Pick one below to enable write-back.
sources-write-bookings-to = Write bookings to:
sources-write-none = None (don't write)
sources-empty = No calendar sources connected. { $link } to check availability.
sources-empty-link-label = Add one

# Dashboard event types listing (templates/dashboard_event_types.html)

event-types-page-title = Event Types
event-types-heading = Event types
event-types-new = New
event-types-badge-disabled = disabled
event-types-badge-internal = internal
event-types-badge-private = private
event-types-badge-resources = resources
event-types-send-invites = Send invites
event-types-duration = { $minutes }min
event-types-mode-collective = collective
event-types-mode-round-robin = round robin
event-types-edit = Edit
event-types-disable = Disable
event-types-enable = Enable
event-types-embed = Embed
event-types-overrides = Overrides
event-types-team-settings = Team settings
event-types-invites = Invites
event-types-view-public = View public page
event-types-view-page = View page
event-types-delete = Delete
event-types-delete-confirm = Delete event type '{ $title }'? This cannot be undone.
event-types-empty = No event types yet. { $link } to start accepting bookings.
event-types-empty-link-label = Create one

# Markdown editor toolbar (templates/settings.html, templates/team_form.html)

editor-bold = Bold (Ctrl+B)
editor-italic = Italic (Ctrl+I)
editor-strikethrough = Strikethrough
editor-code = Inline code
editor-link = Insert link (Ctrl+K)
editor-toggle-preview = Toggle preview
editor-preview = Preview

# Profile and settings (templates/settings.html)

settings-page-title = Settings
settings-heading = Profile & Settings
settings-public-page-label = Your public booking page
settings-copy = Copy
settings-copied = Copied!
settings-open = Open
settings-avatar = Avatar
settings-upload = Upload
settings-remove = Remove
settings-display-name = Display name
settings-display-name-placeholder = Your name
settings-username = Username
settings-username-hint = (used in your booking URL)
settings-username-pattern-title = Lowercase letters, numbers, and dashes only
settings-username-help = Your public booking page:
settings-title = Title
settings-title-placeholder = e.g. Software Engineer, Product Manager
settings-title-help = Shown on your public profile and in the sidebar.
settings-bio = Bio
settings-bio-placeholder = Tell people a bit about yourself...
settings-bio-help = Shown on your public booking page. Supports **bold**, *italic*, ~~strikethrough~~, `code`, and [links](url).
settings-booking-email = Booking email
settings-booking-email-help = This email will appear on your public booking pages and in email notifications. Leave empty to use your login email.
settings-booking-email-warning = Make sure this email exists on your mail provider. If it doesn't, notifications won't be delivered.
settings-timezone = Timezone
settings-timezone-help = Your availability rules and booking times are computed in this timezone.
settings-language = Language
settings-language-auto = Auto (browser default)
settings-language-help = Pick a UI language, or leave on Auto to follow your browser's setting.
settings-dynamic-group = Allow others to include me in dynamic group links
settings-dynamic-group-help = When enabled, other users can create ad-hoc collective meeting URLs that include you (e.g. { $example }).
settings-lend-resource = Lend my calendar access for resource reservations
settings-lend-resource-help = When a booking needs to reserve a shared resource (demo lab, meeting room) that your calendar account can write to, allow calrs to use your stored calendar credentials for that write.
settings-default-availability = Default availability
settings-default-availability-help = Your default working hours. Used for dynamic group links when others include you in a meeting.
settings-copy-to-all = Copy to all days
settings-copy-to-all-title = Copy the first enabled day's windows to all other enabled days
settings-add-window = Add time window
settings-remove-window = Remove window
settings-save = Save settings
settings-appearance = Appearance
settings-theme-system = System
settings-theme-light = Light
settings-theme-dark = Dark

# Sign in (templates/auth/login.html)

login-page-title = Sign in
login-heading = Sign in
login-subtitle = Sign in to your calrs account
login-sso = Sign in with SSO
login-or = or
login-email = Email
login-password = Password
login-submit = Sign in with email
login-no-account = Don't have an account? { $link }
login-register-link = Register

# Registration (templates/auth/register.html)

register-page-title = Register
register-heading = Create account
register-subtitle = Register for a new calrs account
register-domains-limited = Registration is limited to: { $domains }
register-name = Name
register-name-placeholder = Your name
register-email = Email
register-password = Password
register-password-hint = (min. 12 characters)
register-submit = Create account
register-have-account = Already have an account? { $link }
register-signin-link = Sign in

# Authentication errors (src/auth.rs)

auth-error-rate-limited = Too many login attempts. Please try again later.
auth-error-invalid-credentials = Invalid email or password
auth-error-internal = Internal error
auth-error-registration-disabled = Registration is disabled.
auth-error-name-length = Name must be between 1 and 255 characters
auth-error-email-length = Email must be between 1 and 255 characters
auth-error-email-invalid = Please enter a valid email address
auth-error-email-domain = Email domain not allowed
auth-error-password-length = Password must be at least 12 characters
auth-error-email-taken = Email already registered
auth-error-create-failed = Failed to create account

# Calendar source test and write-back setup (templates/source_test.html, templates/source_write_setup.html)

source-test-page-title = Calendar source
source-test-sync-heading = Sync: { $name }
source-test-heading = Connection test
source-write-page-title = Set up calendar write-back
source-write-back = Back to dashboard
source-write-heading = Where should bookings go?
source-write-help = When someone books a meeting with you, calrs can automatically create the event in your calendar. Pick which calendar to write bookings to for { $name }.
source-write-save = Save
source-write-skip = Skip for now
source-write-sync-results = Sync results

source-write-event-count =
    { $count ->
        [one] { $count } event
       *[other] { $count } events
    }

# Date overrides (templates/overrides.html)

overrides-page-title = Date overrides
overrides-heading = Date overrides
overrides-back-teams = Back to teams
overrides-back-event-types = Back to event types
overrides-intro = Add date-specific exceptions for { $title }
overrides-add-heading = Add new override
overrides-date = Date
overrides-type = Override type
overrides-type-blocked = Block entire day
overrides-type-custom = Custom hours
overrides-start-time = Start time
overrides-end-time = End time
overrides-add-submit = Add override
overrides-existing = Existing overrides
overrides-badge-blocked = blocked
overrides-badge-custom = custom hours
overrides-delete = Delete
overrides-delete-confirm = Delete this override?
overrides-empty = No date overrides yet.<br>Use the form above to block specific dates (holidays, days off) or set custom hours.

# Public team page (templates/team_profile.html)

team-profile-subtitle = Pick an event type to book a time.
team-profile-empty = No event types available yet.
