use chrono::{DateTime, FixedOffset};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq)]
pub enum EventStatus {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Hash, PartialEq)]
pub struct SoonToBeIcsEvent {
    pub name: String,
    pub pretty_name: String,
    pub status: EventStatus,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
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

const ICS_SUFFIX: &str = r#"
END:VCALENDAR
"#;

pub fn generate_ics(calendarname: &str, events: &[SoonToBeIcsEvent]) -> String {
    let mut result = String::default();

    result += ICS_PREFIX;
    let calname = format!("X-WR-CALNAME:@HAWHHCalendarBot ({})\n", calendarname);
    result += &calname;
    result += ICS_TIMEZONE;

    let mut lines: Vec<String> = Vec::new();
    for event in events {
        lines.push(event_as_ics_vevent_string(&event));
    }
    result += &lines.join("\n");

    result += ICS_SUFFIX;

    result.replace("\n", "\r\n")
}

fn event_as_ics_vevent_string(event: &SoonToBeIcsEvent) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push("BEGIN:VEVENT".to_owned());
    lines.push("TRANSP:OPAQUE".to_owned());

    lines.push(format!(
        "STATUS:{}",
        match event.status {
            EventStatus::Confirmed => "CONFIRMED",
            _ => "CANCELLED",
        }
        .to_owned()
    ));

    lines.push(format!("SUMMARY:{}", event.pretty_name));
    lines.push(format!(
        "DTSTART;TZID=Europe/Berlin:{}",
        date_to_ics_date(&event.start_time)
    ));
    lines.push(format!(
        "DTEND;TZID=Europe/Berlin:{}",
        date_to_ics_date(&event.end_time)
    ));

    if !event.location.is_empty() {
        lines.push(format!("LOCATION:{}", event.location.replace(",", "\\,")));
    }

    if !event.description.is_empty() {
        lines.push(format!("DESCRIPTION:{}", event.description));
    }

    lines.push("URL;VALUE=URI:https://telegram.me/HAWHHCalendarBot".to_owned());
    lines.push(format!(
        "UID:{}@calendarbot.hawhh.de",
        calculate_event_hash(&event)
    ));
    lines.push("END:VEVENT".to_owned());

    lines.join("\n")
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
    date.format("%Y%m%d %H%M%S").to_string().replace(" ", "T")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ics_date() {
        let date = DateTime::parse_from_rfc3339("2020-08-22T08:30:00+02:00").unwrap();
        let result = date_to_ics_date(&date);
        assert_eq!(result, "20200822T083000")
    }

    #[test]
    fn create_minimal_event_vevent() {
        let event = SoonToBeIcsEvent {
            name: "BTI5-VS".to_owned(),
            pretty_name: "BTI5-VS".to_owned(),
            status: EventStatus::Cancelled,
            start_time: DateTime::parse_from_rfc3339("2020-08-22T08:30:00+02:00").unwrap(),
            end_time: DateTime::parse_from_rfc3339("2020-08-22T11:30:00+02:00").unwrap(),
            description: "".to_owned(),
            location: "".to_owned(),
        };

        let result = event_as_ics_vevent_string(&event);

        assert_eq!(
            result,
            "BEGIN:VEVENT\nTRANSP:OPAQUE\nSTATUS:CANCELLED\nSUMMARY:BTI5-VS\nDTSTART;TZID=Europe/Berlin:20200822T083000\nDTEND;TZID=Europe/Berlin:20200822T113000\nURL;VALUE=URI:https://telegram.me/HAWHHCalendarBot\nUID:cdc823d56e1d56be@calendarbot.hawhh.de\nEND:VEVENT"
        );
    }
}
