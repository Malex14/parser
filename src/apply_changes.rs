#![allow(clippy::non_ascii_literal)]

use crate::generate_ics::{EventStatus, SoonToBeIcsEvent};
use crate::userconfig::{self, Change, RemovedEvents};

pub fn apply_changes(
    events: &mut Vec<SoonToBeIcsEvent>,
    changes: &[Change],
    removed_events: RemovedEvents,
) -> Result<(), String> {
    for change in changes {
        apply_change(events, change, removed_events)?;
    }

    Ok(())
}

fn apply_change(
    events: &mut Vec<SoonToBeIcsEvent>,
    change: &Change,
    removed_events: RemovedEvents,
) -> Result<(), String> {
    let mut iter = events.iter();
    let change_date = userconfig::parse_change_date(&change.date)
        .map_err(|err| format!("failed to parse change date {} Error: {err}", change.date))?;
    if change.add {
        let end_time = change
            .endtime
            .ok_or("parse change add has no end_time specified")?;
        let end_time = change_date
            .date_naive()
            .and_time(end_time)
            .and_local_timezone(change_date.timezone())
            .unwrap();

        #[allow(clippy::option_if_let_else)]
        events.push(SoonToBeIcsEvent {
            name: change.name.clone(),
            pretty_name: if let Some(namesuffix) = &change.namesuffix {
                format!("{} {namesuffix}", change.name)
            } else {
                change.name.clone()
            },
            status: EventStatus::Confirmed,
            start_time: change_date,
            end_time,
            alert_minutes_before: None,
            description: "Dies ist eine zusätzliche Veranstaltung welche manuell von dir über den Telegram Bot hinzufügt wurde.".to_owned(),
            location: change.room.clone().unwrap_or_default(),
        });
    } else if let Some(i) = iter.position(|o| o.name == change.name && o.start_time == change_date)
    {
        let mut event = &mut events[i];
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

        if let Some(namesuffix) = &change.namesuffix {
            event.pretty_name = format!("{} {namesuffix}", event.pretty_name);
        }

        if let Some(room) = &change.room {
            event.location = room.clone();
        }

        if let Some(time) = &change.starttime {
            event.start_time = change_date
                .date_naive()
                .and_time(*time)
                .and_local_timezone(change_date.timezone())
                .unwrap();
        }

        if let Some(time) = &change.endtime {
            event.end_time = change_date
                .date_naive()
                .and_time(*time)
                .and_local_timezone(change_date.timezone())
                .unwrap();
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
            description: String::new(),
            location: String::new(),
        },
        SoonToBeIcsEvent {
            name: "BTI5-VSP/01".to_owned(),
            pretty_name: "BTI5-VSP/01".to_owned(),
            status: EventStatus::Confirmed,
            start_time: chrono::DateTime::parse_from_rfc3339("2020-05-14T08:15:00+02:00").unwrap(),
            end_time: chrono::DateTime::parse_from_rfc3339("2020-05-14T11:15:00+02:00").unwrap(),
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
        date: "2020-05-15T13:37".to_owned(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
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
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Removed).unwrap();
    assert_eq!(events.len(), 1);
}

#[test]
fn remove_event_gets_marked_as_cancelled() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].status, EventStatus::Cancelled);
}

#[test]
fn remove_event_gets_emoji_prefix() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: false,
        remove: true,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Emoji).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].pretty_name, "🚫 BTI5-VSP/01");
}

#[test]
fn namesuffix_is_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: false,
        remove: false,
        starttime: None,
        endtime: None,
        namesuffix: Some("whatever".to_owned()),
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].pretty_name, "BTI5-VSP/01 whatever");
}

#[test]
fn room_is_overwritten() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: false,
        remove: false,
        starttime: None,
        endtime: None,
        namesuffix: None,
        room: Some("whereever".to_owned()),
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].location, "whereever");
}

#[test]
fn starttime_changed() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-14T06:15".to_owned(),
        add: false,
        remove: false,
        starttime: Some(chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap()),
        endtime: None,
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
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
        add: false,
        remove: false,
        starttime: None,
        endtime: Some(chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap()),
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events[1].end_time.to_rfc3339(), "2020-05-14T08:30:00+02:00");
}

#[test]
fn event_added() {
    let mut events = generate_events();
    let change = Change {
        name: "BTI5-VSP/01".to_owned(),
        date: "2020-05-30T08:00".to_owned(),
        add: true,
        remove: false,
        starttime: None,
        endtime: Some(chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
        namesuffix: None,
        room: None,
    };
    apply_change(&mut events, &change, RemovedEvents::Cancelled).unwrap();
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].name, "BTI5-VSP/01");
    assert_eq!(
        events[2].start_time.to_rfc3339(),
        "2020-05-30T10:00:00+02:00"
    );
    assert_eq!(events[2].end_time.to_rfc3339(), "2020-05-30T10:30:00+02:00");
    assert_eq!(events[2].location, "");
}
