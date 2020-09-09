use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use chrono_tz::Europe::Berlin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserconfigFile {
    pub chat: Chat,
    pub config: Userconfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
}

#[derive(Debug, PartialEq)]
pub enum RemovedEvents {
    Cancelled,
    Removed,
    Emoji,
}

fn parse_removed_events(string: &str) -> Result<RemovedEvents, String> {
    match string {
        "cancelled" => Ok(RemovedEvents::Cancelled),
        "removed" => Ok(RemovedEvents::Removed),
        "emoji" => Ok(RemovedEvents::Emoji),
        _ => Err(format!("could not parse removed events {}", string)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Userconfig {
    pub calendarfile_suffix: String,
    pub changes: Vec<Change>,
    pub events: Vec<String>,
    removed_events: Option<String>, // See enum RemovedEvents
}

impl Userconfig {
    pub fn removed_events(&self) -> Result<RemovedEvents, String> {
        match &self.removed_events {
            Some(string) => parse_removed_events(&string),
            None => Ok(RemovedEvents::Cancelled),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Change {
    pub add: Option<bool>,
    pub name: String,
    pub date: String,
    pub remove: Option<bool>,
    pub namesuffix: Option<String>,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub room: Option<String>,
}

pub fn parse_change_date(raw: &str) -> Result<DateTime<FixedOffset>, String> {
    let tless = raw.replace('T', " ");
    let naive = NaiveDateTime::parse_from_str(&tless, "%Y-%m-%d %H:%M")
        .map_err(|err| format!("parse_datetime failed naive {} Error: {}", raw, err))?;
    let date_time = Berlin.from_utc_datetime(&naive);
    let fixed_offset = DateTime::parse_from_rfc3339(&date_time.to_rfc3339())
        .map_err(|err| format!("parse_datetime failed fixed_offset {} Error: {}", raw, err))?;
    Ok(fixed_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn can_parse_change_date_from_utc_to_local() {
        let actual = parse_change_date("2020-07-01T06:30").unwrap();
        let string = actual.to_rfc3339();
        assert_eq!(string, "2020-07-01T08:30:00+02:00");
    }

    #[test]
    fn can_deserialize_chat() -> Result<(), serde_json::Error> {
        let test: Chat = serde_json::from_str(
            r#"{"id": 133766642, "is_bot": false, "first_name": "Peter", "last_name": "Parker", "username": "Spiderman", "language_code": "en"}"#,
        )?;

        assert_eq!(test.id, 133766642);
        assert_eq!(test.first_name, "Peter");

        Ok(())
    }

    #[test]
    fn error_on_userconfig_without_calendarfile_suffix() -> Result<(), String> {
        let test: Result<Userconfig, serde_json::Error> =
            serde_json::from_str(r#"{"changes": [], "events": []}"#);

        match test {
            Err(error) => {
                assert_eq!(error.is_data(), true);
                Ok(())
            }
            _ => Err("should fail".to_owned()),
        }
    }

    #[test]
    fn can_deserialize_minimal_userconfig() -> Result<(), serde_json::Error> {
        let test: Userconfig = serde_json::from_str(
            r#"{"calendarfileSuffix": "123qwe", "changes": [], "events": []}"#,
        )?;

        assert_eq!(test.calendarfile_suffix, "123qwe");
        assert_eq!(test.changes.len(), 0);
        assert_eq!(test.events.len(), 0);
        assert_eq!(test.removed_events, None);

        Ok(())
    }

    #[test]
    fn can_deserialize_userconfig_with_events() -> Result<(), serde_json::Error> {
        let test: Userconfig = serde_json::from_str(
            r#"{"calendarfileSuffix": "123qwe", "changes": [], "events": ["BTI1-TI", "BTI5-VS"]}"#,
        )?;

        assert_eq!(test.calendarfile_suffix, "123qwe");
        assert_eq!(test.changes.len(), 0);
        assert_eq!(test.events, ["BTI1-TI", "BTI5-VS"]);
        assert_eq!(test.removed_events, None);

        Ok(())
    }

    #[test]
    fn can_deserialize_minimal_change() -> Result<(), serde_json::Error> {
        let test: Change = serde_json::from_str(r#"{"name": "Tree", "date": "2020-12-20T22:04"}"#)?;
        assert_eq!(test.add, None);
        assert_eq!(test.name, "Tree");
        assert_eq!(test.date, "2020-12-20T22:04");
        assert_eq!(test.remove, None);
        assert_eq!(test.namesuffix, None);
        assert_eq!(test.starttime, None);
        assert_eq!(test.endtime, None);
        assert_eq!(test.room, None);

        Ok(())
    }

    #[test]
    fn removed_events_can_be_parsed() {
        assert_eq!(
            parse_removed_events("cancelled"),
            Ok(RemovedEvents::Cancelled)
        );
        assert_eq!(parse_removed_events("removed"), Ok(RemovedEvents::Removed));
        assert_eq!(parse_removed_events("emoji"), Ok(RemovedEvents::Emoji));
        assert_eq!(
            parse_removed_events("wha"),
            Err("could not parse removed events wha".to_owned())
        );
    }
}
