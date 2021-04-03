use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use chrono::DateTime;
use serde::Deserialize;

use crate::generate_ics::{EventStatus, SoonToBeIcsEvent};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EventEntry {
    pub name: String,
    pub location: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
}

pub const FOLDER: &str = "eventfiles";

pub fn read(name: &str) -> Result<Vec<EventEntry>, String> {
    let filename = name.replace("/", "-");
    let path = Path::new(FOLDER).join(filename + ".json");
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read: {}", err))?;
    let event_entries: Vec<EventEntry> =
        serde_json::from_str(&content).map_err(|err| format!("failed to parse: {}", err))?;

    Ok(event_entries)
}

impl TryFrom<EventEntry> for SoonToBeIcsEvent {
    type Error = String;

    fn try_from(event: EventEntry) -> Result<Self, Self::Error> {
        Ok(SoonToBeIcsEvent {
            name: event.name.to_owned(),
            pretty_name: event.name.to_owned(),
            status: EventStatus::Confirmed,
            start_time: DateTime::parse_from_rfc3339(&event.start_time).map_err(|err| {
                format!(
                    "parse event start time failed {} Error: {}",
                    event.start_time, err
                )
            })?,
            end_time: DateTime::parse_from_rfc3339(&event.end_time).map_err(|err| {
                format!(
                    "parse event end time failed {} Error: {}",
                    event.end_time, err
                )
            })?,
            description: event.description.to_owned(),
            location: event.location.to_owned(),
        })
    }
}

#[test]
fn can_deserialize_event_entry() -> Result<(), serde_json::Error> {
    let test: EventEntry = serde_json::from_str(
        r#"{"Name": "BTI1-TI", "Location": "1060", "Description": "Dozent: HTM", "StartTime": "08:30", "EndTime": "11:30"}"#,
    )?;

    assert_eq!(test.name, "BTI1-TI");
    assert_eq!(test.location, "1060");
    assert_eq!(test.description, "Dozent: HTM");
    assert_eq!(test.start_time, "08:30");
    assert_eq!(test.end_time, "11:30");

    Ok(())
}
