use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EventEntry {
    pub name: String,
    pub location: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
}

pub fn read_event(name: &str) -> Result<Vec<EventEntry>, String> {
    let filename = name.replace("/", "-");
    let path = Path::new("eventfiles").join(filename + ".json");
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read event file {} {}", name, err))?;
    let event_entries: Vec<EventEntry> = serde_json::from_str(&content)
        .map_err(|err| format!("failed to parse event file {} {}", name, err))?;

    Ok(event_entries)
}

pub fn read_events(event_names: &[String]) -> Result<Vec<EventEntry>, String> {
    let mut result: Vec<EventEntry> = Vec::new();
    for name in event_names {
        let mut events = read_event(name)?;
        result.append(&mut events);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
}
