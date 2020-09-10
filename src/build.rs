use crate::apply_changes::apply_changes;
use crate::changestatus::{Changestatus, Changetype};
use crate::events::read_events;
use crate::generate_ics::generate_ics;
use crate::userconfig::UserconfigFile;
use std::fs;
use std::path::Path;

pub struct Buildresult {
    pub changestatus: Changestatus,
    pub filename: String,
}

pub const FOLDER: &str = "calendars";

pub fn ensure_directory() -> Result<(), std::io::Error> {
    fs::create_dir_all(FOLDER)
}

pub fn build(content: &UserconfigFile) -> Result<Changestatus, String> {
    Ok(build_interal(&content)?.changestatus)
}

fn build_interal(content: &UserconfigFile) -> Result<Buildresult, String> {
    let user_id = content.chat.id;
    let first_name = &content.chat.first_name;
    let ics_filename = format!("{}-{}.ics", user_id, &content.config.calendarfile_suffix);
    let path = Path::new(FOLDER).join(&ics_filename);

    if content.config.events.is_empty() {
        let changetype = if path.exists() {
            fs::remove_file(&path).map_err(|err| {
                format!(
                    "failed to remove calendar with now 0 events {} {} Error: {}",
                    user_id, first_name, err
                )
            })?;
            Changetype::Removed
        } else {
            Changetype::Skipped
        };

        return Ok(Buildresult {
            filename: ics_filename,
            changestatus: Changestatus {
                name: first_name.to_owned(),
                changetype,
            },
        });
    }

    let user_events = read_events(&content.config.events);
    let removed_events = content.config.removed_events().map_err(|err| {
        format!(
            "failed to parse type of removed_events for user {} {} Error: {}",
            user_id, first_name, err
        )
    })?;
    let mut result_events = apply_changes(&user_events, &content.config.changes, &removed_events)
        .map_err(|err| {
            format!(
                "failed to apply changes for user {} {} Error: {}",
                user_id, first_name, err
            )
        })?;
    result_events.sort_by_cached_key(|event| event.start_time);
    let ics_content = generate_ics(first_name, &result_events);

    let changetype = if let Ok(current_content) = fs::read_to_string(&path) {
        if ics_content == current_content {
            Changetype::Same
        } else {
            Changetype::Changed
        }
    } else {
        Changetype::Added
    };

    if changetype != Changetype::Same {
        fs::write(&path, &ics_content).map_err(|err| {
            format!(
                "failed to write ics file content for user {} {} Error: {}",
                user_id, first_name, err
            )
        })?;
    }

    Ok(Buildresult {
        filename: ics_filename,
        changestatus: Changestatus {
            name: first_name.to_owned(),
            changetype,
        },
    })
}

pub fn build_all_remove_rest(list: &[UserconfigFile]) -> Result<Vec<Changestatus>, String> {
    let mut changestati: Vec<Changestatus> = Vec::new();
    let mut superfluous = get_existing_files()
        .map_err(|err| format!("failed to read calendars dir for cleanup {}", err))?;

    for content in list {
        match build_interal(&content) {
            Ok(filechange) => {
                if let Some(index) = superfluous
                    .iter()
                    .position(|x| x.as_ref() == filechange.filename)
                {
                    superfluous.remove(index);
                }

                changestati.push(filechange.changestatus);
            }
            Err(error) => println!(
                "build for {} {} failed. Error: {}",
                content.chat.id, content.chat.first_name, error
            ),
        }
    }

    for filename in &superfluous {
        let path = Path::new(FOLDER).join(filename);
        fs::remove_file(path).map_err(|err| {
            format!(
                "failed to remove superfluous calendar file {} Error: {}",
                filename, err
            )
        })?;

        changestati.push(Changestatus {
            name: filename.to_owned(),
            changetype: Changetype::Removed,
        })
    }

    Ok(changestati)
}

fn get_existing_files() -> Result<Vec<String>, std::io::Error> {
    let mut list: Vec<String> = Vec::new();
    for maybe_entry in fs::read_dir(FOLDER)? {
        let filename = maybe_entry?
            .file_name()
            .into_string()
            .expect("filename contains something that can not be read easily with rust");

        list.push(filename.to_owned());
    }

    Ok(list)
}
