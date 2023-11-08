#![allow(clippy::non_ascii_literal)]

use crate::generate_ics::{EventStatus, SoonToBeIcsEvent};
use crate::userconfig::{Change, RemovedEvents};

pub fn apply_changes(
    events: &mut Vec<SoonToBeIcsEvent>,
    changes: Vec<Change>,
    removed_events: RemovedEvents,
) -> Result<(), String> {
    for change in changes {
        apply_change(events, change, removed_events)?;
    }

    Ok(())
}

#[allow(clippy::suspicious_operation_groupings)]
fn apply_change(
    events: &mut Vec<SoonToBeIcsEvent>,
    change: Change,
    removed_events: RemovedEvents,
) -> Result<(), String> {
    let mut iter = events.iter();
    if change.add {
        let end_time = change
            .endtime
            .ok_or("change add has no end_time specified")?;
        let end_time = change.date.date().and_time(end_time);

        #[allow(clippy::option_if_let_else)]
        events.push(SoonToBeIcsEvent {
            pretty_name: if let Some(namesuffix) = change.namesuffix {
                format!("{} {namesuffix}", change.name)
            } else {
                change.name.clone()
            },
            name: change.name,
            status: EventStatus::Confirmed,
            start_time: change.date,
            end_time,
            alert_minutes_before: None,
            description: "Dies ist eine zusätzliche Veranstaltung welche manuell von dir über den Telegram Bot hinzufügt wurde.".to_owned(),
            location: change.room.unwrap_or_default(),
        });
    } else if let Some(i) = iter.position(|o| o.name == change.name && o.start_time == change.date)
    {
        let event = &mut events[i];
        if change.remove {
            match removed_events {
                RemovedEvents::Cancelled => event.status = EventStatus::Cancelled,
                RemovedEvents::Emoji => event.pretty_name = format!("🚫 {}", event.pretty_name),
                RemovedEvents::Removed => {
                    events.remove(i);
                    return Ok(());
                }
            }
        }

        if let Some(namesuffix) = change.namesuffix {
            event.pretty_name = format!("{} {namesuffix}", event.pretty_name);
        }

        if let Some(room) = change.room {
            event.location = room;
        }

        if let Some(time) = change.starttime {
            event.start_time = change.date.date().and_time(time);
        }

        if let Some(time) = change.endtime {
            event.end_time = change.date.date().and_time(time);
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
            start_time: chrono::NaiveDate::from_ymd_opt(2020, 4, 2)
                .unwrap()
                .and_hms_opt(8, 15, 0)
                .unwrap(),
            end_time: chrono::NaiveDate::from_ymd_opt(2020, 4, 2)
                .unwrap()
                .and_hms_opt(11, 15, 0)
                .unwrap(),
            alert_minutes_before: None,
            description: String::new(),
            location: String::new(),
        },
        SoonToBeIcsEvent {
            name: "BTI5-VSP/01".to_owned(),
            pretty_name: "BTI5-VSP/01".to_owned(),
            status: EventStatus::Confirmed,
            start_time: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
                .unwrap()
                .and_hms_opt(8, 15, 0)
                .unwrap(),
            end_time: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
                .unwrap()
                .and_hms_opt(11, 15, 0)
                .unwrap(),
            alert_minutes_before: None,
            description: String::new(),
            location: String::new(),
        },
    ]
}

#[test]
fn non_existing_event_of_change_is_skipped() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VS".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 15)
            .unwrap()
            .and_hms_opt(13, 37, 0)
            .unwrap(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
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
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Removed).unwrap();
    assert_eq!(events.len(), 1);
}

#[test]
fn remove_event_gets_marked_as_cancelled() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].status, EventStatus::Cancelled);
}

#[test]
fn remove_event_gets_emoji_prefix() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Emoji).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].pretty_name, "🚫 BTI5-VSP/01");
}

#[test]
fn namesuffix_is_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: false,
        starttime: None,
        endtime: None,
        namesuffix: Some("whatever".to_owned()),
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].pretty_name, "BTI5-VSP/01 whatever");
}

#[test]
fn room_is_overwritten() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: false,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: Some("whereever".to_owned()),
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].location, "whereever");
}

#[test]
fn starttime_changed() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: false,
        starttime: Some(chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap()),
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(
        events[1].start_time,
        chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 30, 0)
            .unwrap()
    );
}

#[test]
fn endtime_changed() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 15, 0)
            .unwrap(),
        add: false,
        remove: false,
        starttime: None,
        endtime: Some(chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap()),
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(
        events[1].end_time,
        chrono::NaiveDate::from_ymd_opt(2020, 5, 14)
            .unwrap()
            .and_hms_opt(8, 30, 0)
            .unwrap()
    );
}

#[test]
fn event_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(2020, 5, 30)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(),
        add: true,
        remove: false,
        starttime: None,
        endtime: Some(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].name, "BTI5-VSP/01");
    assert_eq!(
        events[2].start_time,
        chrono::NaiveDate::from_ymd_opt(2020, 5, 30)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    );
    assert_eq!(
        events[2].end_time,
        chrono::NaiveDate::from_ymd_opt(2020, 5, 30)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap()
    );
    assert_eq!(events[2].location, "");
}
