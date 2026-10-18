//! Shared rendering: HTML and plain-text layout, ICS generation, time conversion, email parts.

use super::*;

// --- HTML email template helpers ---

pub(super) fn h(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// --- Cascade email palette ---
//
// Email is the only Cascade surface that renders inside someone else's client,
// so it cannot use CSS custom properties or webfonts. These constants are the
// hard-coded equivalent of the product tokens. Light only: dark-mode handling
// in mail clients is unreliable, so the shell is designed for light and
// degrades gracefully when a client force-inverts it.
pub(super) const OCEAN: &str = "#1A3A4A";
pub(super) const OCEAN_DEEP: &str = "#0E2530";
pub(super) const OCEAN_LIGHT: &str = "#E8F0F3";
/// 2x2 emblem: solid stand-in for Deep Ocean at 55% over the page background.
/// Real `opacity` is unreliable in Outlook, so the steps are pre-blended.
pub(super) const OCEAN_STEP: &str = "#7D8D95";
pub(super) const CORAL: &str = "#E07A5F";
pub(super) const PAGE_BG: &str = "#F5F3F0";
pub(super) const CARD_BG: &str = "#FFFFFF";
pub(super) const BORDER: &str = "#E8E6E3";
pub(super) const BORDER_OCEAN: &str = "#C9D6DB";
pub(super) const TEXT: &str = "#22201D";
pub(super) const TEXT_2ND: &str = "#5F5D5A";
pub(super) const TEXT_MUTED: &str = "#757370";
pub(super) const FOOTER_TEXT: &str = "#9FB4BD";
pub(super) const SUCCESS: &str = "#2D6A4F";
pub(super) const WARNING: &str = "#7A5C18";
pub(super) const ERROR: &str = "#B03A2B";

pub(super) const FONT_BODY: &str = "'Plus Jakarta Sans',-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif";
pub(super) const FONT_DISPLAY: &str = "'Bricolage Grotesque','Plus Jakarta Sans',-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif";
pub(super) const FONT_MONO: &str = "'DM Mono',ui-monospace,SFMono-Regular,Menlo,Consolas,monospace";

pub(super) struct EmailRow {
    pub(super) label: String,
    pub(super) value: String,
}

/// Semantic accent role. Cascade emails share one shell; the role only tints
/// the 3px rule at the top of the card and the small status mark beside the
/// greeting. It is never a full-width coloured band, and it never varies by
/// email type for decoration's sake.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Accent {
    /// Default. Informational: invitations, reminders, test sends, claims.
    Ocean,
    /// A booking now exists and is confirmed.
    Success,
    /// Awaiting a decision, or about to happen.
    Warning,
    /// Cancelled, declined, or otherwise undone.
    Error,
}

impl Accent {
    /// Colour of the top rule and the status mark.
    pub(super) fn color(self) -> &'static str {
        match self {
            Accent::Ocean => OCEAN,
            Accent::Success => SUCCESS,
            Accent::Warning => WARNING,
            Accent::Error => ERROR,
        }
    }
}

/// How an action button renders. `Primary` is a Deep Ocean fill; `Ghost` is a
/// bordered outline for the secondary half of a pair (decline, cancel).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ActionStyle {
    Primary,
    Ghost,
}

pub(super) struct EmailAction {
    pub(super) label: String,
    pub(super) url: String,
    pub(super) style: ActionStyle,
}

/// The Cascade lockup: the 2x2 emblem plus the wordmark, left-aligned above
/// the card.
///
/// The emblem is built from table cells with background colours rather than an
/// inline SVG or a hosted `<img>`. Inline SVG is dropped by Outlook and by
/// several webmail sanitisers, and a hosted PNG needs a configured public base
/// URL plus the recipient choosing to load remote images — neither is
/// guaranteed. Table cells always paint. Outlook's Word engine ignores
/// `border-radius`, so the emblem degrades to four hard squares, which still
/// reads as the mark.
pub(super) fn brand_lockup() -> String {
    let cell = |color: &str| {
        format!(
            "<td width=\"13\" height=\"13\" style=\"width:13px;height:13px;line-height:13px;font-size:0;background:{color};border-radius:4px;\">&nbsp;</td>"
        )
    };
    let gap = "<td width=\"2\" style=\"width:2px;font-size:0;line-height:0;\">&nbsp;</td>";
    format!(
        "<table role=\"presentation\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"border-collapse:collapse;\">\
           <tr>\
             <td width=\"28\" style=\"width:28px;padding:0 10px 0 0;vertical-align:middle;\">\
               <table role=\"presentation\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"28\" style=\"width:28px;border-collapse:collapse;\">\
                 <tr>{tl}{gap}{tr}</tr>\
                 <tr><td colspan=\"3\" height=\"2\" style=\"height:2px;font-size:0;line-height:0;\">&nbsp;</td></tr>\
                 <tr>{bl}{gap}{br}</tr>\
               </table>\
             </td>\
             <td style=\"vertical-align:middle;font-family:{FONT_DISPLAY};font-size:18px;font-weight:700;letter-spacing:-0.02em;color:{OCEAN};\">Cascade</td>\
           </tr>\
         </table>",
        tl = cell(OCEAN),
        tr = cell(OCEAN_STEP),
        bl = cell(OCEAN_STEP),
        br = cell(CORAL),
    )
}

/// A bulletproof button: a one-cell table with a `bgcolor` and a padded `<a>`,
/// so it paints in Outlook as well as in webmail.
pub(super) fn action_button(action: &EmailAction) -> String {
    let (bg, fg, border, pad) = match action.style {
        ActionStyle::Primary => (OCEAN, "#FFFFFF", "none", "12px 22px"),
        ActionStyle::Ghost => (CARD_BG, OCEAN, BORDER_OCEAN, "11px 21px"),
    };
    let border_style = if border == "none" {
        String::new()
    } else {
        format!("border:1px solid {border};")
    };
    format!(
        "<table role=\"presentation\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"border-collapse:separate;\">\
           <tr><td bgcolor=\"{bg}\" style=\"background:{bg};{border_style}border-radius:8px;\">\
             <a href=\"{url}\" class=\"cx-btn\" style=\"display:inline-block;padding:{pad};font-family:{FONT_BODY};font-size:13px;font-weight:600;line-height:1;color:{fg};text-decoration:none;border-radius:8px;\">{label}</a>\
           </td></tr>\
         </table>",
        url = h(&action.url),
        label = h(&action.label),
    )
}

pub(super) fn render_html_email(
    greeting: &str,
    message: &str,
    accent: Accent,
    rows: &[EmailRow],
    footer_note: Option<&str>,
) -> String {
    render_html_email_in("en", greeting, message, accent, rows, footer_note, &[])
}

#[cfg(test)]
pub(super) fn render_html_email_with_actions(
    greeting: &str,
    message: &str,
    accent: Accent,
    rows: &[EmailRow],
    footer_note: Option<&str>,
    actions: &[EmailAction],
) -> String {
    render_html_email_in("en", greeting, message, accent, rows, footer_note, actions)
}

/// Render the HTML body for an email written in `lang`. The tag ends up in
/// `<html lang>`, so mail clients pick the right spell checker and screen
/// readers the right voice (#209). Only the admin SMTP test email still goes
/// through the English wrapper above. The Cascade shell (lockup, card and
/// footer band) is the same in every language.
pub(super) fn render_html_email_in(
    lang: &str,
    greeting: &str,
    message: &str,
    accent: Accent,
    rows: &[EmailRow],
    footer_note: Option<&str>,
    actions: &[EmailAction],
) -> String {
    // Cascade: the footer band's note is the one piece of shell copy, and it
    // is written in the reader's language like the rest of the email.
    let footer_note_text = h(&t(lang, "email-footer-note"));
    let lang = h(crate::i18n::html_lang(lang));
    let accent_color = accent.color();

    let details_html = if rows.is_empty() {
        String::new()
    } else {
        let last = rows.len() - 1;
        let mut detail_rows = String::new();
        for (i, row) in rows.iter().enumerate() {
            let rule = if i == last {
                String::new()
            } else {
                format!("border-bottom:1px solid {BORDER};")
            };
            detail_rows.push_str(&format!(
                "<tr>\
                   <td class=\"cx-label\" width=\"34%\" style=\"width:34%;padding:11px 14px;{rule}background:{OCEAN_LIGHT};font-family:{FONT_MONO};font-size:10px;letter-spacing:0.07em;text-transform:uppercase;color:{TEXT_MUTED};vertical-align:top;\">{label}</td>\
                   <td class=\"cx-value\" style=\"padding:11px 14px;{rule}font-family:{FONT_BODY};font-size:14px;line-height:1.5;color:{TEXT};vertical-align:top;\">{value}</td>\
                 </tr>",
                label = h(&row.label),
                value = h(&row.value),
            ));
        }
        format!(
            "<table role=\"presentation\" width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"width:100%;margin:22px 0 0;border:1px solid {BORDER};border-radius:8px;border-collapse:separate;overflow:hidden;\">\
               {detail_rows}\
             </table>"
        )
    };

    let actions_html = if actions.is_empty() {
        String::new()
    } else {
        let cells: Vec<String> = actions
            .iter()
            .map(|a| {
                format!(
                    "<td class=\"cx-btn-cell\" style=\"padding:0 10px 0 0;\">{}</td>",
                    action_button(a)
                )
            })
            .collect();
        format!(
            "<table role=\"presentation\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\" style=\"margin:24px 0 0;border-collapse:collapse;\"><tr>{}</tr></table>",
            cells.join("")
        )
    };

    let footer_html = footer_note
        .map(|n| {
            format!(
                "<p style=\"margin:20px 0 0;font-family:{FONT_BODY};font-size:13px;line-height:1.55;color:{TEXT_2ND};\">{}</p>",
                h(n)
            )
        })
        .unwrap_or_default();

    let lockup = brand_lockup();

    format!(
        r##"<!DOCTYPE html>
<html lang="{lang}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1.0">
<meta name="color-scheme" content="light">
<meta name="supported-color-schemes" content="light">
<style>
  @media only screen and (max-width:620px) {{
    .cx-pad {{ padding: 24px 20px 26px !important; }}
    .cx-foot {{ padding: 20px !important; }}
    .cx-btn-cell {{ display:block !important; width:100% !important; padding:0 0 10px 0 !important; }}
    .cx-btn-cell table {{ width:100% !important; }}
    .cx-btn {{ display:block !important; text-align:center !important; }}
  }}
</style>
</head>
<body style="margin:0;padding:0;background:{PAGE_BG};font-family:{FONT_BODY};color:{TEXT};-webkit-font-smoothing:antialiased;">
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:{PAGE_BG};width:100%;">
<tr><td align="center" style="padding:32px 16px;">
  <table role="presentation" width="600" cellpadding="0" cellspacing="0" border="0" style="width:600px;max-width:100%;">
    <!-- Header lockup -->
    <tr><td align="left" style="padding:0 4px 16px;">{lockup}</td></tr>
    <!-- Card -->
    <tr><td>
      <table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="width:100%;background:{CARD_BG};border:1px solid {BORDER};border-radius:12px;border-collapse:separate;">
        <!-- Accent rule -->
        <tr><td height="3" style="height:3px;line-height:3px;font-size:0;background:{accent_color};border-radius:12px 12px 0 0;">&nbsp;</td></tr>
        <!-- Content -->
        <tr><td class="cx-pad" style="padding:28px 32px 32px;">
          <table role="presentation" cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;">
            <tr>
              <td width="8" style="width:8px;padding:0 9px 0 0;vertical-align:middle;">
                <table role="presentation" cellpadding="0" cellspacing="0" border="0" width="8" style="width:8px;border-collapse:collapse;">
                  <tr><td height="8" style="width:8px;height:8px;line-height:8px;font-size:0;background:{accent_color};border-radius:8px;">&nbsp;</td></tr>
                </table>
              </td>
              <td style="vertical-align:middle;font-family:{FONT_BODY};font-size:13px;line-height:1.4;color:{TEXT_2ND};">{greeting}</td>
            </tr>
          </table>
          <p style="margin:12px 0 0;font-family:{FONT_DISPLAY};font-size:20px;line-height:1.35;font-weight:700;letter-spacing:-0.01em;color:{TEXT};">{message}</p>
          {details_html}
          {actions_html}
          {footer_html}
        </td></tr>
        <!-- Footer band -->
        <tr><td class="cx-foot" style="padding:20px 32px;background:{OCEAN_DEEP};border-radius:0 0 12px 12px;">
          <p style="margin:0;font-family:{FONT_DISPLAY};font-size:14px;font-weight:700;letter-spacing:-0.01em;color:#FFFFFF;">Cascade</p>
          <p style="margin:6px 0 0;font-family:{FONT_BODY};font-size:12px;line-height:1.55;color:{FOOTER_TEXT};">{footer_note_text}</p>
          <p style="margin:6px 0 0;font-family:{FONT_MONO};font-size:11px;letter-spacing:0.04em;"><a href="https://cascade.tax" style="color:{OCEAN_LIGHT};text-decoration:none;">cascade.tax</a></p>
        </td></tr>
      </table>
    </td></tr>
  </table>
</td></tr>
</table>
</body>
</html>"##
    )
}

pub(super) fn build_multipart_body(plain: &str, html: &str) -> MultiPart {
    MultiPart::alternative()
        .singlepart(SinglePart::plain(plain.to_string()))
        .singlepart(
            SinglePart::builder()
                .header(ContentType::parse("text/html; charset=UTF-8").unwrap())
                .body(html.to_string()),
        )
}

// --- ICS generation ---

/// Sanitize a value for use in an ICS field.
/// Strips CR/LF to prevent ICS injection (RFC 5545 field breakout), and
/// escapes the TEXT specials of RFC 5545 §3.3.11. The backslash goes first:
/// left raw, `C:\new folder` reads as a line break and `a\;b` as an escaped
/// semicolon (#266). Callers join already-sanitized pieces with a literal
/// `\n`, which is why that separator is never passed through here.
pub(super) fn sanitize_ics(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\r', "")
        .replace('\n', " ")
        .replace(';', "\\;")
        .replace(',', "\\,")
}

/// Sanitize a value for use in an ICS *parameter*, such as `CN=`.
///
/// A parameter value is not a TEXT value and takes no backslash escapes.
/// RFC 5545 §3.1 defines it as either `paramtext`, which excludes `;`, `:`,
/// `,` and DQUOTE, or a quoted-string. Escaping a semicolon as `\;` the way
/// `sanitize_ics` does leaves the raw `;` in place, a strict parser reads it
/// as the end of the parameter, and the whole VEVENT is rejected: Yandex 360
/// answers 400 Bad Request to a booking whose guest name contains one, and
/// the event never reaches the calendar (#163).
///
/// So quote the value when it carries a delimiter, and spell the characters a
/// quoted-string cannot hold with the RFC 6868 caret escapes. The carets only
/// appear for input containing `"`, `^` or a newline; a parser predating
/// RFC 6868 renders those literally, which is a cosmetic loss in a display
/// name rather than the parse failure this replaces.
pub(super) fn sanitize_ics_param(value: &str) -> String {
    let escaped = value
        .replace('^', "^^")
        .replace('"', "^'")
        .replace("\r\n", "^n")
        .replace(['\r', '\n'], "^n");
    if escaped.contains([';', ':', ',']) {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

/// Fold every content line longer than 75 octets (RFC 5545 §3.1): the line
/// is broken with CRLF followed by a single space, and the space counts
/// towards the next line's 75. Breaks fall on UTF-8 character boundaries, so
/// an accented name or an em dash in a SUMMARY is never split mid-character.
/// Applied once to the whole output of each generator, after every line is
/// assembled, so no field can be forgotten. Strict parsers reject unfolded
/// lines, and DESCRIPTION grows with the guest phone and notes (#267).
pub fn fold_ics(ics: &str) -> String {
    const MAX: usize = 75;
    let mut out = String::with_capacity(ics.len() + ics.len() / MAX * 3);
    for (i, line) in ics.split("\r\n").enumerate() {
        if i > 0 {
            out.push_str("\r\n");
        }
        let mut width = 0;
        for c in line.chars() {
            let len = c.len_utf8();
            if width + len > MAX {
                out.push_str("\r\n ");
                width = 1;
            }
            out.push(c);
            width += len;
        }
    }
    out
}

/// Convert date + start/end times from a guest timezone to UTC ICS format (YYYYMMDDTHHMMSSZ).
/// Falls back to floating time (no Z) if timezone parsing fails.
pub(super) fn convert_to_utc(
    date: &str,
    start_time: &str,
    end_time: &str,
    timezone: &str,
) -> (String, String) {
    let fallback_start = format!(
        "{}T{}00",
        date.replace('-', ""),
        start_time.replace(':', "")
    );
    let fallback_end = format!("{}T{}00", date.replace('-', ""), end_time.replace(':', ""));

    let tz: Tz = match timezone.parse() {
        Ok(t) => t,
        Err(_) => return (fallback_start, fallback_end),
    };

    let start_naive = match NaiveDateTime::parse_from_str(
        &format!("{} {}:00", date, start_time),
        "%Y-%m-%d %H:%M:%S",
    ) {
        Ok(dt) => dt,
        Err(_) => return (fallback_start, fallback_end),
    };
    let end_naive = match NaiveDateTime::parse_from_str(
        &format!("{} {}:00", date, end_time),
        "%Y-%m-%d %H:%M:%S",
    ) {
        Ok(dt) => dt,
        Err(_) => return (fallback_start, fallback_end),
    };

    use chrono::TimeZone;
    let start_utc = match tz.from_local_datetime(&start_naive).earliest() {
        Some(dt) => dt.with_timezone(&chrono::Utc),
        None => return (fallback_start, fallback_end),
    };
    let end_utc = match tz.from_local_datetime(&end_naive).earliest() {
        Some(dt) => dt.with_timezone(&chrono::Utc),
        None => return (fallback_start, fallback_end),
    };

    (
        start_utc.format("%Y%m%dT%H%M%SZ").to_string(),
        end_utc.format("%Y%m%dT%H%M%SZ").to_string(),
    )
}

/// Convert a (date, start_time, end_time) tuple from `from_tz` into `to_tz`,
/// returning the wall-clock equivalents.
///
/// Returns `(date, start_time, end_time)` formatted as `YYYY-MM-DD` / `HH:MM`.
/// The date can roll over (e.g. LA 22:00 → Paris 07:00 next day), so the
/// returned date may differ from the input. If either timezone fails to parse
/// or the local time is invalid (DST gap), returns `None` and the caller
/// should fall back to displaying the original values.
pub(super) fn convert_time_between_tz(
    date: &str,
    start_time: &str,
    end_time: &str,
    from_tz: &str,
    to_tz: &str,
) -> Option<(String, String, String)> {
    use chrono::TimeZone;

    let from: Tz = from_tz.parse().ok()?;
    let to: Tz = to_tz.parse().ok()?;

    let start_naive =
        NaiveDateTime::parse_from_str(&format!("{} {}:00", date, start_time), "%Y-%m-%d %H:%M:%S")
            .ok()?;
    let end_naive =
        NaiveDateTime::parse_from_str(&format!("{} {}:00", date, end_time), "%Y-%m-%d %H:%M:%S")
            .ok()?;

    let start_target = from
        .from_local_datetime(&start_naive)
        .earliest()?
        .with_timezone(&to);
    let end_target = from
        .from_local_datetime(&end_naive)
        .earliest()?
        .with_timezone(&to);

    Some((
        start_target.format("%Y-%m-%d").to_string(),
        start_target.format("%H:%M").to_string(),
        end_target.format("%H:%M").to_string(),
    ))
}

/// Format a stored `HH:MM` value for human-facing email copy. Keep the stored
/// value unchanged everywhere else (notably ICS generation and timezone math).
pub(super) fn display_time(value: &str) -> String {
    NaiveTime::parse_from_str(value, "%H:%M")
        .map(|time| time.format("%-I:%M %p").to_string())
        .unwrap_or_else(|_| value.to_string())
}

pub(super) fn display_time_range(start_time: &str, end_time: &str, timezone: &str) -> String {
    let range = format!(
        "{} \u{2013} {}",
        display_time(start_time),
        display_time(end_time)
    );
    if timezone.is_empty() {
        range
    } else {
        format!("{} ({})", range, timezone)
    }
}

/// Build the date + time strings to display in a host-targeted email.
///
/// When `host_timezone` is set and differs from `guest_timezone`, the times are
/// converted into the host's zone. Otherwise the original guest-zone values are
/// kept. The returned `time_display` always includes a `(TZ)` suffix so the
/// host can tell which zone they're looking at.
pub(crate) fn host_time_display(
    date: &str,
    start_time: &str,
    end_time: &str,
    guest_timezone: &str,
    host_timezone: &str,
) -> (String, String) {
    if !host_timezone.is_empty() && host_timezone != guest_timezone {
        if let Some((host_date, host_start, host_end)) =
            convert_time_between_tz(date, start_time, end_time, guest_timezone, host_timezone)
        {
            return (
                host_date,
                display_time_range(&host_start, &host_end, host_timezone),
            );
        }
    }

    let tz_label = if !guest_timezone.is_empty() {
        guest_timezone
    } else if !host_timezone.is_empty() {
        host_timezone
    } else {
        ""
    };

    let time_display = display_time_range(start_time, end_time, tz_label);

    (date.to_string(), time_display)
}

/// New rows carry exact endpoints; do not reconstruct an ambiguous guest
/// wall clock or assume the end falls on the start date.
pub(super) fn host_time_display_exact(
    date: &str,
    start_time: &str,
    end_time: &str,
    guest_timezone: &str,
    host_timezone: &str,
    utc_times: Option<&(String, String)>,
) -> (String, String) {
    let zone = if host_timezone.is_empty() {
        guest_timezone
    } else {
        host_timezone
    };
    if let (Some((start, end)), Ok(tz)) = (utc_times, zone.parse::<chrono_tz::Tz>()) {
        let parse = |v: &str| {
            chrono::NaiveDateTime::parse_from_str(v, "%Y%m%dT%H%M%SZ")
                .ok()
                .map(|v| v.and_utc().with_timezone(&tz))
        };
        if let (Some(start), Some(end)) = (parse(start), parse(end)) {
            return (
                start.format("%Y-%m-%d").to_string(),
                display_time_range(
                    &start.format("%H:%M").to_string(),
                    &end.format("%H:%M").to_string(),
                    zone,
                ),
            );
        }
    }
    host_time_display(date, start_time, end_time, guest_timezone, host_timezone)
}

/// Generate an .ics VCALENDAR string for a booking
pub fn generate_ics(details: &BookingDetails, method: &str) -> String {
    generate_ics_impl(details, method, false)
}

/// ICS for CalDAV write-back (RFC 4791: no METHOD). ATTENDEE lines carry
/// SCHEDULE-AGENT=CLIENT (RFC 6638 §7.1) so the CalDAV server does not
/// run its own scheduling: calrs already sends the invitations over SMTP,
/// and servers like Fastmail would otherwise send a duplicate iMIP invite
/// (in UTC) for every written booking. Email .ics attachments keep plain
/// ATTENDEE;RSVP=TRUE so mail clients still offer "Add to calendar".
pub fn generate_ics_caldav(details: &BookingDetails) -> String {
    generate_ics_impl(details, "", true)
}

fn generate_ics_impl(
    details: &BookingDetails,
    method: &str,
    schedule_agent_client: bool,
) -> String {
    // Cascade: the title is the event name only; attendees are listed on the
    // invitation itself, not in the calendar title.
    let summary = sanitize_ics(&details.event_title);
    // CN= is a parameter, not a TEXT value: it needs quoting, not backslashes.
    let host_name = sanitize_ics_param(&details.host_name);
    let guest_name = sanitize_ics_param(&details.guest_name);
    let host_email = sanitize_ics(&details.host_email);
    let guest_email = sanitize_ics(&details.guest_email);
    let location_line = details
        .location
        .as_ref()
        .map(|l| format!("LOCATION:{}\r\n", sanitize_ics(l)))
        .unwrap_or_default();
    // The phone goes only into the host's own calendar copy (#261): an email
    // attachment also reaches additional attendees, who have no use for it.
    let phone = details
        .guest_phone
        .as_ref()
        .filter(|p| schedule_agent_client && !p.trim().is_empty())
        .map(|p| {
            sanitize_ics(&format!(
                "{} {}",
                t(host_lang(details), "email-detail-phone"),
                p
            ))
        });
    let notes = details
        .notes
        .as_ref()
        .filter(|n| !n.trim().is_empty())
        .map(|n| sanitize_ics(n));
    let description: Vec<String> = phone.into_iter().chain(notes).collect();
    let description_line = if description.is_empty() {
        String::new()
    } else {
        // `\n` is the escaped line break of an RFC 5545 TEXT value.
        format!("DESCRIPTION:{}\r\n", description.join("\\n"))
    };
    let valarm = details
        .reminder_minutes
        .filter(|&m| m > 0)
        .map(|m| {
            format!(
                "BEGIN:VALARM\r\n\
                 TRIGGER:-PT{m}M\r\n\
                 ACTION:DISPLAY\r\n\
                 DESCRIPTION:Reminder\r\n\
                 END:VALARM\r\n"
            )
        })
        .unwrap_or_default();
    let sa = if schedule_agent_client {
        ";SCHEDULE-AGENT=CLIENT"
    } else {
        ""
    };
    let additional_attendee_lines: String = details
        .additional_attendees
        .iter()
        .chain(details.host_attendees.iter())
        .map(|email| format!("ATTENDEE{sa};RSVP=TRUE:mailto:{}\r\n", sanitize_ics(email)))
        .collect();
    let dtstamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    // Convert guest-timezone times to UTC for the ICS
    let (dtstart, dtend) = details.utc_times.clone().unwrap_or_else(|| {
        convert_to_utc(
            &details.date,
            &details.start_time,
            &details.end_time,
            &details.guest_timezone,
        )
    });
    fold_ics(&format!(
        "BEGIN:VCALENDAR\r\n\
         VERSION:2.0\r\n\
         PRODID:-//Cascade//Calendar//EN\r\n\
         {method_line}\
         BEGIN:VEVENT\r\n\
         UID:{uid}\r\n\
         DTSTAMP:{dtstamp}\r\n\
         SEQUENCE:{sequence}\r\n\
         DTSTART:{dtstart}\r\n\
         DTEND:{dtend}\r\n\
         SUMMARY:{summary}\r\n\
         {description_line}\
         {location_line}\
         ORGANIZER;CN={host_name}:mailto:{host_email}\r\n\
         ATTENDEE{sa};CN={guest_name};RSVP=TRUE:mailto:{guest_email}\r\n\
         {additional_attendee_lines}\
         STATUS:CONFIRMED\r\n\
         {valarm}\
         END:VEVENT\r\n\
         END:VCALENDAR\r\n",
        method_line = if method.is_empty() {
            String::new()
        } else {
            format!("METHOD:{method}\r\n")
        },
        uid = details.uid,
        dtstamp = dtstamp,
        sequence = details.sequence,
        dtstart = dtstart,
        dtend = dtend,
        summary = summary,
        description_line = description_line,
        location_line = location_line,
        host_name = host_name,
        host_email = host_email,
        guest_name = guest_name,
        guest_email = guest_email,
        additional_attendee_lines = additional_attendee_lines,
    ))
}

/// Generate an .ics VCALENDAR for cancellation (METHOD:CANCEL)
pub(super) fn generate_cancel_ics(details: &CancellationDetails) -> String {
    // Cascade: the title is the event name only; attendees are listed on the
    // invitation itself, not in the calendar title.
    let summary = sanitize_ics(&details.event_title);
    // CN= is a parameter, not a TEXT value: it needs quoting, not backslashes.
    let host_name = sanitize_ics_param(&details.host_name);
    let guest_name = sanitize_ics_param(&details.guest_name);
    let host_email = sanitize_ics(&details.host_email);
    let guest_email = sanitize_ics(&details.guest_email);
    // RFC 5546 3.2.5: a CANCEL names the attendees it removes the event for,
    // here everyone the invite listed (#267).
    let additional_attendee_lines: String = details
        .additional_attendees
        .iter()
        .map(|email| format!("ATTENDEE:mailto:{}\r\n", sanitize_ics(email)))
        .collect();
    let dtstamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let (dtstart, dtend) = details.utc_times.clone().unwrap_or_else(|| {
        convert_to_utc(
            &details.date,
            &details.start_time,
            &details.end_time,
            &details.guest_timezone,
        )
    });
    fold_ics(&format!(
        "BEGIN:VCALENDAR\r\n\
         VERSION:2.0\r\n\
         PRODID:-//Cascade//Calendar//EN\r\n\
         METHOD:CANCEL\r\n\
         BEGIN:VEVENT\r\n\
         UID:{uid}\r\n\
         DTSTAMP:{dtstamp}\r\n\
         SEQUENCE:{sequence}\r\n\
         DTSTART:{dtstart}\r\n\
         DTEND:{dtend}\r\n\
         SUMMARY:{summary}\r\n\
         ORGANIZER;CN={host_name}:mailto:{host_email}\r\n\
         ATTENDEE;CN={guest_name}:mailto:{guest_email}\r\n\
         {additional_attendee_lines}\
         STATUS:CANCELLED\r\n\
         END:VEVENT\r\n\
         END:VCALENDAR\r\n",
        uid = details.uid,
        dtstamp = dtstamp,
        sequence = details.sequence,
        dtstart = dtstart,
        dtend = dtend,
        summary = summary,
        host_name = host_name,
        host_email = host_email,
        guest_name = guest_name,
        guest_email = guest_email,
        additional_attendee_lines = additional_attendee_lines,
    ))
}

// --- Composition shared by the translated emails ---

pub(super) fn row(label: String, value: impl Into<String>) -> EmailRow {
    EmailRow {
        label,
        value: value.into(),
    }
}

/// Append a row only when the value is there.
pub(super) fn push_row(rows: &mut Vec<EmailRow>, lang: &str, key: &str, value: Option<&String>) {
    if let Some(v) = value {
        rows.push(row(t(lang, key), v.clone()));
    }
}

/// Event, date, time and guest: the rows every host-facing email opens with.
pub(super) fn host_rows(
    lang: &str,
    event: &str,
    date: &str,
    time: &str,
    guest_name: &str,
    guest_email: &str,
) -> Vec<EmailRow> {
    vec![
        row(t(lang, "confirmed-detail-event"), event),
        row(t(lang, "confirmed-detail-date"), date),
        row(t(lang, "confirmed-detail-time"), time),
        row(
            t(lang, "common-detail-guest"),
            format!("{} <{}>", guest_name, guest_email),
        ),
    ]
}

/// "date at time", for rows that carry both in one value.
pub(super) fn date_at_time(lang: &str, date: &str, time: &str) -> String {
    ta(lang, "email-date-at-time", [("date", date), ("time", time)])
}

/// Approve and decline links for a pending booking, when a token and a
/// public base URL are both known.
pub(super) fn approval_urls(
    confirm_token: Option<&str>,
    base_url: Option<&str>,
) -> Option<(String, String)> {
    let (token, url) = (confirm_token?, base_url?);
    let base = url.trim_end_matches('/');
    Some((
        format!("{}/booking/approve/{}", base, token),
        format!("{}/booking/decline/{}", base, token),
    ))
}

/// Plain-text lines and HTML buttons for approve/decline, or the dashboard
/// hint when the email cannot carry links.
pub(super) fn approval_actions(
    lang: &str,
    urls: Option<(String, String)>,
) -> (Vec<String>, Vec<EmailAction>) {
    match urls {
        Some((approve, decline)) => (
            vec![
                ta(lang, "email-host-approve-line", [("url", &approve)]),
                ta(lang, "email-host-decline-line", [("url", &decline)]),
            ],
            vec![
                EmailAction {
                    label: t(lang, "email-action-approve"),
                    url: approve,
                    style: ActionStyle::Primary,
                },
                EmailAction {
                    label: t(lang, "email-action-decline"),
                    url: decline,
                    style: ActionStyle::Ghost,
                },
            ],
        ),
        None => (vec![t(lang, "email-host-approval-dashboard")], vec![]),
    }
}

/// An email made of a heading, one message, a details table, optional
/// buttons and a closing note, all in `lang`. The plain-text body carries the
/// same content, with `plain_lines` (the URLs the HTML shows as buttons)
/// after the details.
pub(super) struct EmailParts<'a> {
    pub(super) lang: &'a str,
    pub(super) subject: String,
    pub(super) heading: String,
    pub(super) message: String,
    pub(super) accent: Accent,
    pub(super) rows: Vec<EmailRow>,
    pub(super) plain_lines: Vec<String>,
    pub(super) footer_plain: Option<String>,
    pub(super) footer_html: Option<String>,
    pub(super) actions: Vec<EmailAction>,
}

impl EmailParts<'_> {
    pub(super) fn compose(self) -> ComposedEmail {
        let mut plain = format!("{}\n\n{}\n\n", self.heading, self.message);
        for r in &self.rows {
            plain.push_str(&format!("{} {}\n", r.label, r.value));
        }
        plain.push('\n');
        if !self.plain_lines.is_empty() {
            plain.push_str(&self.plain_lines.join("\n"));
            plain.push_str("\n\n");
        }
        if let Some(note) = &self.footer_plain {
            plain.push_str(note);
            plain.push_str("\n\n");
        }
        plain.push_str(&t(self.lang, "email-signature"));
        let html = render_html_email_in(
            self.lang,
            &h(&self.heading),
            &h(&self.message),
            self.accent,
            &self.rows,
            self.footer_html.as_deref(),
            &self.actions,
        );
        ComposedEmail {
            subject: self.subject,
            plain,
            html,
        }
    }
}

/// Send a composed email, with an optional `(filename, ics, method)`
/// calendar attachment.
pub(super) async fn send_composed(
    config: &SmtpConfig,
    sender: Sender<'_>,
    to: Mailbox,
    mail: ComposedEmail,
    ics: Option<(&str, String, &str)>,
) -> Result<()> {
    let body = build_multipart_body(&mail.plain, &mail.html);
    let builder = config.message_from(sender)?.to(to).subject(mail.subject);
    let email = match ics {
        Some((filename, ics, method)) => {
            let attachment = Attachment::new(filename.to_string()).body(
                ics,
                ContentType::parse(&format!("text/calendar; method={method}; charset=UTF-8"))?,
            );
            builder.multipart(MultiPart::mixed().multipart(body).singlepart(attachment))?
        }
        None => builder.multipart(body)?,
    };
    send_email(config, email).await
}

/// Subject and bodies of an email, built apart from sending so tests can
/// read exactly what the recipient would get.
#[derive(Clone)]
pub(super) struct ComposedEmail {
    pub(super) subject: String,
    pub(super) plain: String,
    pub(super) html: String,
}

/// "[status] event, date" subject for guest emails that report a state
/// rather than carry an invite, built from the translated status label and
/// the same neutral "event, date" pair the confirmation uses.
pub(super) fn status_subject(lang: &str, status_key: &str, event: &str, date: &str) -> String {
    format!(
        "[{}] {}",
        t(lang, status_key),
        ta(
            lang,
            "email-confirm-subject",
            [("event", event), ("date", date)]
        )
    )
}
