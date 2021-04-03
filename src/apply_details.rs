use crate::generate_ics::SoonToBeIcsEvent;
use crate::userconfig::EventDetails;

pub fn apply_details(event: &mut SoonToBeIcsEvent, details: &EventDetails) {
    if let Some(notes) = &details.notes {
        if !notes.is_empty() {
            event.description = if event.description.is_empty() {
                notes.to_owned()
            } else {
                format!("{}\n\n{}", event.description, notes)
            };
        }
    }
}

#[cfg(test)]
fn create_event(description: &str) -> SoonToBeIcsEvent {
    SoonToBeIcsEvent {
        name: "BTI5-VSP/01".to_owned(),
        pretty_name: "BTI5-VSP/01".to_owned(),
        status: crate::generate_ics::EventStatus::Confirmed,
        start_time: chrono::DateTime::parse_from_rfc3339("2020-04-02T08:15:00+02:00").unwrap(),
        end_time: chrono::DateTime::parse_from_rfc3339("2020-04-02T11:15:00+02:00").unwrap(),
        description: description.to_owned(),
        location: "".to_owned(),
    }
}

#[cfg(test)]
fn check_description(notes: Option<&str>, event_description: &str, expected: &str) {
    let details = EventDetails {
        notes: notes.map(std::borrow::ToOwned::to_owned),
    };
    let mut event = create_event(event_description);
    apply_details(&mut event, &details);
    assert_eq!(event.description, expected);
}

#[test]
fn no_note_no_description() {
    check_description(None, "", "");
}

#[test]
fn no_note_some_description() {
    check_description(None, "bla", "bla");
}

#[test]
fn some_note_no_description() {
    check_description(Some("bla"), "", "bla");
}

#[test]
fn some_note_some_description() {
    check_description(Some("foo"), "bar", "bar\n\nfoo");
}
