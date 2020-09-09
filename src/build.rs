use crate::apply_changes::apply_changes;
use crate::events::read_events;
use crate::generate_ics::generate_ics;
use crate::userconfig::UserconfigFile;
use std::fs;
use std::path::Path;

pub fn ensure_directory() -> Result<(), std::io::Error> {
    fs::create_dir_all("calendars")
}

pub fn build(content: &UserconfigFile) -> Result<(), String> {
    let user_id = content.chat.id;
    let first_name = &content.chat.first_name;

    let user_events = read_events(&content.config.events).map_err(|err| {
        format!(
            "failed to read events for user {} {} {}",
            user_id, first_name, err
        )
    })?;
    let removed_events = content.config.removed_events().map_err(|err| {
        format!(
            "failed to parse type of removed_events for user {} {} {}",
            user_id, first_name, err
        )
    })?;
    let result_events = apply_changes(&user_events, &content.config.changes, &removed_events)
        .map_err(|err| {
            format!(
                "failed to apply changes for user {} {} {}",
                user_id, first_name, err
            )
        })?;
    let ics_content = generate_ics(first_name, &result_events);

    let ics_filename = format!("{}-{}.ics", user_id, &content.config.calendarfile_suffix);
    let path = Path::new("calendars").join(&ics_filename);
    fs::write(path, &ics_content).map_err(|err| {
        format!(
            "failed to write ics file content for user {} {} {}",
            user_id, first_name, err
        )
    })?;

    println!("build {} {}", content.chat.first_name, ics_content.len());

    Ok(())
}
