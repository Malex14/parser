#![allow(clippy::non_ascii_literal)]

use crate::generate_ics::{EventStatus, SoonToBeIcsEvent};
use crate::userconfig::{self, Change, RemovedEvents};
use chrono::NaiveTime;

pub fn apply_changes(
    events: &mut Vec<SoonToBeIcsEvent>,
    changes: &[Change],
    removed_events: &RemovedEvents,
) -> Result<(), String> {
    for change in changes {
        apply_change(events, change, &removed_events)?;
    }

    Ok(())
}

fn apply_change(
    events: &mut Vec<SoonToBeIcsEvent>,
    change: &Change,
    removed_events: &RemovedEvents,
) -> Result<(), String> {
    let mut iter = events.iter();
    let change_date = userconfig::parse_change_date(&change.date)
        .map_err(|err| format!("failed to parse change date {} Error: {}", change.date, err))?;
    if change.add == Some(true) {
        let end_time = change
            .endtime
            .to_owned()
            .ok_or("parse change add has no end_time specified")?;
        let time = NaiveTime::parse_from_str(&end_time, "%H:%M")
            .map_err(|err| format!("parse change end time failed {} Error: {}", end_time, err))?;
        let end_time = change_date.date().and_time(time).unwrap();

        events.push(SoonToBeIcsEvent {
            name: change.name.to_owned(),
            pretty_name: if let Some(namesuffix) = &change.namesuffix {
                format!("{} {}", change.name, namesuffix)
            } else {
                change.name.to_owned()
            },
            status: EventStatus::Confirmed,
            start_time: change_date.to_owned(),
            end_time,
            alert_minutes_before: None,
            description: "Dies ist eine zusätzliche Veranstaltung welche manuell von dir über den Telegram Bot hinzufügt wurde.".to_owned(),
            location: change.room.to_owned().unwrap_or_default(),
        });
    } else if let Some(i) = iter.position(|o| o.name == change.name && o.start_time == change_date)
    {
        let mut event = &mut events[i];
        if change.remove == Some(true) {
            match removed_events {
                RemovedEvents::Cancelled => event.status = EventStatus::Cancelled,
                RemovedEvents::Emoji => event.pretty_name = format!("🚫 {}", event.pretty_name),
                RemovedEvents::Removed => {
                    events.remove(i);
                    return Ok(());
                }
            }
        }

        if let Some(namesuffix) = &change.namesuffix {
            event.pretty_name = format!("{} {}", event.pretty_name, namesuffix);
        }

        if let Some(room) = &change.room {
            event.location = room.to_owned();
        }

        if let Some(start_time) = &change.starttime {
            let time = NaiveTime::parse_from_str(&start_time, "%H:%M").map_err(|err| {
                format!(
                    "parse change start time failed {} Error: {}",
                    start_time, err
                )
            })?;
            event.start_time = change_date.date().and_time(time).unwrap();
        }

        if let Some(end_time) = &change.endtime {
            let time = NaiveTime::parse_from_str(&end_time, "%H:%M").map_err(|err| {
                format!("parse change end time failed {} Error: {}", end_time, err)
            })?;
            event.end_time = change_date.date().and_time(time).unwrap();
        }
    } else {
        // Event for this change doesnt exist.
        // This not nice but the TelegramBot has to solve this via user feedback.
    }

    Ok(())
}

#[cfg(test)]
fn generate_events() -> Vec<SoonToBeIcsEvent> {
    vec![
        SoonToBeIcsEvent {
            name: "BTI5-VSP/01".to_owned(),
            pretty_name: "BTI5-VSP/01".to_owned(),
            status: EventStatus::Confirmed,
            start_time: chrono::DateTime::parse_from_rfc3339("2020-04-02T08:15:00+02:00").unwrap(),
            end_time: chrono::DateTime::parse_from_rfc3339("2020-04-02T11:15:00+02:00").unwrap(),
            alert_minutes_before: None,
            description: "".to_owned(),
            location: "".to_owned(),
        },
        SoonToBeIcsEvent {
            name: "BTI5-VSP/01".to_owned(),
            pretty_name: "BTI5-VSP/01".to_owned(),
            status: EventStatus::Confirmed,
            start_time: chrono::DateTime::parse_from_rfc3339("2020-05-14T08:15:00+02:00").unwrap(),
            end_time: chrono::DateTime::parse_from_rfc3339("2020-05-14T11:15:00+02:00").unwrap(),
            alert_minutes_before: None,
            description: "".to_owned(),
            location: "".to_owned(),
        },
    ]
}

#[test]
fn non_existing_event_of_change_is_skipped() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VS".to_owned(),
        date: "2020-05-15T13:37".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
        remove: Some(true),
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 2);

    let expected = generate_events();
    assert_eq!(events[0], expected[0]);
    assert_eq!(events[1], expected[1]);
}

#[test]
fn remove_event_is_removed_completly() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
        remove: Some(true),
    };
    apply_change(&mut events, &change, &RemovedEvents::Removed).unwrap();
    assert_eq!(events.len(), 1);
}

#[test]
fn remove_event_gets_marked_as_cancelled() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
        remove: Some(true),
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].status, EventStatus::Cancelled);
}

#[test]
fn remove_event_gets_emoji_prefix() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
        remove: Some(true),
    };
    apply_change(&mut events, &change, &RemovedEvents::Emoji).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].pretty_name, "🚫 BTI5-VSP/01");
}

#[test]
fn namesuffix_is_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: Some("whatever".to_owned()),
        room: None,
        remove: None,
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].pretty_name, "BTI5-VSP/01 whatever");
}

#[test]
fn room_is_overwritten() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: Some("whereever".to_owned()),
        remove: None,
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].location, "whereever");
}

#[test]
fn starttime_changed() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: Some("08:30".to_owned()),
        endtime: None,
        namesuffix: None,
        room: None,
        remove: None,
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(
        events[1].start_time.to_rfc3339(),
        "2020-05-14T08:30:00+02:00"
    );
}

#[test]
fn endtime_changed() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: None,
        starttime: None,
        endtime: Some("08:30".to_owned()),
        namesuffix: None,
        room: None,
        remove: None,
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].end_time.to_rfc3339(), "2020-05-14T08:30:00+02:00");
}

#[test]
fn event_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-30T08:00".to_owned(),
        add: Some(true),
        starttime: None,
        endtime: Some("10:30".to_owned()),
        namesuffix: None,
        room: None,
        remove: None,
    };
    apply_change(&mut events, &change, &RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].name, "BTI5-VSP/01");
    assert_eq!(
        events[2].start_time.to_rfc3339(),
        "2020-05-30T10:00:00+02:00"
    );
    assert_eq!(events[2].end_time.to_rfc3339(), "2020-05-30T10:30:00+02:00");
    assert_eq!(events[2].location, "");
}
