use chrono::{DateTime, FixedOffset};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Write;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SoonToBeIcsEvent {
    pub name: String,
    pub pretty_name: String,
    pub status: EventStatus,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
    pub alert_minutes_before: Option<u16>,
    pub description: String,
    pub location: String,
}

const ICS_PREFIX: &str = r#"BEGIN:VCALENDAR
VERSION:2.0
METHOD:PUBLISH
PRODID:https://calendarbot.hawhh.de
"#;

const ICS_TIMEZONE: &str = r#"BEGIN:VTIMEZONE
TZID:Europe/Berlin
BEGIN:DAYLIGHT
TZOFFSETFROM:+0100
RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU
DTSTART:19810329T020000
TZNAME:CEST
TZOFFSETTO:+0200
END:DAYLIGHT
BEGIN:STANDARD
TZOFFSETFROM:+0200
RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU
DTSTART:19961027T030000
TZNAME:CET
TZOFFSETTO:+0100
END:STANDARD
END:VTIMEZONE
"#;

const ICS_SUFFIX: &str = "END:VCALENDAR\n";

pub fn generate_ics(calendarname: &str, events: &[SoonToBeIcsEvent]) -> String {
    let mut result = String::default();

    result += ICS_PREFIX;
    _ = writeln!(result, "X-WR-CALNAME:@HAWHHCalendarBot ({calendarname})");
    result += ICS_TIMEZONE;

    for event in events {
        event_as_ics_vevent_string(&mut result, event);
    }

    result += ICS_SUFFIX;

    result.replace('\n', "\r\n")
}

fn event_as_ics_vevent_string(output: &mut String, event: &SoonToBeIcsEvent) {
    *output += "BEGIN:VEVENT\n";
    *output += "TRANSP:OPAQUE\n";

    _ = writeln!(
        output,
        "STATUS:{}",
        match event.status {
            EventStatus::Confirmed => "CONFIRMED",
            EventStatus::Cancelled => "CANCELLED",
        }
        .to_owned()
    );

    _ = writeln!(
        output,
        "SUMMARY:{}",
        string_to_ical_escaped_text(&event.pretty_name)
    );
    _ = writeln!(
        output,
        "DTSTART;TZID=Europe/Berlin:{}",
        date_to_ics_date(&event.start_time)
    );
    _ = writeln!(
        output,
        "DTEND;TZID=Europe/Berlin:{}",
        date_to_ics_date(&event.end_time)
    );

    if !event.location.is_empty() {
        _ = writeln!(
            output,
            "LOCATION:{}",
            string_to_ical_escaped_text(&event.location)
        );
    }

    if !event.description.is_empty() {
        _ = writeln!(
            output,
            "DESCRIPTION:{}",
            string_to_ical_escaped_text(&event.description)
        );
    }

    *output += "URL;VALUE=URI:https://telegram.me/HAWHHCalendarBot\n";
    _ = writeln!(
        output,
        "UID:{}@calendarbot.hawhh.de",
        calculate_event_hash(event)
    );

    if let Some(minutes_before) = event.alert_minutes_before {
        create_valarm(output, minutes_before);
    }

    *output += "END:VEVENT\n";
}

/// escape according to <https://www.kanzaki.com/docs/ical/text.html>
fn string_to_ical_escaped_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace(',', "\\,")
        .replace(';', "\\;")
        .replace('\n', "\\n")
}

fn calculate_event_hash(event: &SoonToBeIcsEvent) -> String {
    format!("{:x}", calculate_hash(&event))
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn date_to_ics_date(date: &DateTime<FixedOffset>) -> String {
    date.format("%Y%m%d %H%M%S").to_string().replace(' ', "T")
}

/// <https://www.kanzaki.com/docs/ical/valarm.html>
fn create_valarm(output: &mut String, minutes_before: u16) {
    _ = writeln!(
        output,
        "BEGIN:VALARM\nTRIGGER:-PT{}\nACTION:AUDIO\nEND:VALARM",
        minutes_to_ical_duration(minutes_before)
    );
}

/// <https://www.kanzaki.com/docs/ical/duration-t.html>
fn minutes_to_ical_duration(minutes_before: u16) -> String {
    let hours = minutes_before / 60;
    let minutes = minutes_before % 60;
    if hours > 0 && minutes > 0 {
        format!("{hours:02}H{minutes:02}M")
    } else if hours > 0 {
        format!("{hours:02}H")
    } else {
        format!("{minutes:02}M")
    }
}

#[test]
fn parse_ics_date() {
    let date = DateTime::parse_from_rfc3339("2020-08-22T08:30:00+02:00").unwrap();
    let result = date_to_ics_date(&date);
    assert_eq!(result, "20200822T083000");
}

#[test]
fn create_minimal_event_vevent() {
    let event = SoonToBeIcsEvent {
        name: "BTI5-VS".to_owned(),
        pretty_name: "BTI5-VS".to_owned(),
        status: EventStatus::Cancelled,
        start_time: DateTime::parse_from_rfc3339("2020-08-22T08:30:00+02:00").unwrap(),
        end_time: DateTime::parse_from_rfc3339("2020-08-22T11:30:00+02:00").unwrap(),
        alert_minutes_before: None,
        description: String::new(),
        location: String::new(),
    };

    let mut result = String::new();
    event_as_ics_vevent_string(&mut result, &event);
    assert_eq!(
        result,
        "BEGIN:VEVENT\nTRANSP:OPAQUE\nSTATUS:CANCELLED\nSUMMARY:BTI5-VS\nDTSTART;TZID=Europe/Berlin:20200822T083000\nDTEND;TZID=Europe/Berlin:20200822T113000\nURL;VALUE=URI:https://telegram.me/HAWHHCalendarBot\nUID:1e64a94de608b194@calendarbot.hawhh.de\nEND:VEVENT\n"
    );
}

#[test]
fn create_valarm_example() {
    let mut output = String::new();
    create_valarm(&mut output, 10);
    assert_eq!(
        output,
        "BEGIN:VALARM\nTRIGGER:-PT10M\nACTION:AUDIO\nEND:VALARM\n"
    );
}

#[test]
fn minutes_to_ical_duration_examples() {
    assert_eq!(minutes_to_ical_duration(0), "00M");
    assert_eq!(minutes_to_ical_duration(10), "10M");
    assert_eq!(minutes_to_ical_duration(30), "30M");
    assert_eq!(minutes_to_ical_duration(60), "01H");
    assert_eq!(minutes_to_ical_duration(90), "01H30M");
    assert_eq!(minutes_to_ical_duration(120), "02H");
}
