use std::fs;
use std::path::Path;

use serde::Deserialize;

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

fn read_one(name: &str) -> Result<Vec<EventEntry>, String> {
    let filename = name.replace("/", "-");
    let path = Path::new(FOLDER).join(filename + ".json");
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read: {}", err))?;
    let event_entries: Vec<EventEntry> =
        serde_json::from_str(&content).map_err(|err| format!("failed to parse: {}", err))?;

    Ok(event_entries)
}

pub fn read(event_names: &[String]) -> Vec<EventEntry> {
    let mut result: Vec<EventEntry> = Vec::new();
    for name in event_names {
        match read_one(name) {
            Ok(mut events) => result.append(&mut events),
            Err(err) => println!("skip event {:32} {}", name, err),
        }
    }

    result
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
