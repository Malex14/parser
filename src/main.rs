use crate::build::build_all_remove_rest;
use crate::changestatus::create_change_summary;
use crate::changestatus::Changestatus;
use crate::watchcat::Watchcat;
use std::thread::sleep;
use std::time::Duration;

mod apply_changes;
mod build;
mod changestatus;
mod events;
mod generate_ics;
mod userconfig;
mod userconfigs;
mod watchcat;

fn main() {
    build::ensure_directory().unwrap();
    println!("Begin build all configs…");

    let all = userconfigs::load_all().expect("failed to load all userconfigs");
    let changes = build_all_remove_rest(&all).expect("failed to build all initial userconfigs");
    println!(
        "{}",
        create_change_summary(&changes, &changestatus::SHOW_ALL)
    );

    println!("Finished building all configs. Engage watchcats…\n");

    let event_watcher = Watchcat::new(events::FOLDER).unwrap();
    let userconfig_watcher = Watchcat::new(userconfigs::FOLDER).unwrap();

    loop {
        let mut event_changes: Vec<String> = Vec::new();
        event_changes.append(&mut event_watcher.get_changed_filenames());
        if !event_changes.is_empty() {
            println!("eventfile change detected… ");
            sleep(Duration::from_secs(15));
            event_changes.append(&mut event_watcher.get_changed_filenames());
            println!("changed ({:3}): {:?}", event_changes.len(), event_changes);

            match do_all() {
                Ok(summary) => println!("{}", summary),
                Err(err) => println!("failed to build all {}", err),
            }
        }

        for filename in userconfig_watcher.get_changed_filenames() {
            println!("userconfig changed {:>16}… ", filename);
            match do_specific(&filename) {
                Ok(change) => println!("{:?} {}", change.changetype, change.name),
                Err(err) => println!("{}", err),
            }
        }

        sleep(Duration::from_secs(5));
    }
}

fn do_all() -> Result<String, String> {
    let all = userconfigs::load_all()?;
    let changes = build_all_remove_rest(&all)?;
    Ok(create_change_summary(
        &changes,
        &changestatus::SHOW_INTERESTING,
    ))
}

fn do_specific(userconfig_filename: &str) -> Result<Changestatus, String> {
    let config = userconfigs::load_specific(&userconfig_filename)?;
    build::build(&config)
}
