use std::thread::sleep;
use std::time::Duration;

use crate::changestatus::{write_change_summary, Changestatus, Changetype};
use crate::watchcat::Watchcat;

mod apply_changes;
mod apply_details;
mod changestatus;
mod events;
mod generate_ics;
mod output_files;
mod userconfig;
mod userconfigs;
mod watchcat;

fn main() {
    output_files::ensure_directory().expect("should be able to create output directory");
    let mut stdout = std::io::stdout();
    println!("Begin build all configs...");

    let all = userconfigs::load_all();
    let changes = output_files::all_remove_rest(all)
        .expect("should be able to build all initial userconfigs");
    _ = write_change_summary(&mut stdout, changes, Changetype::ALL);

    println!("Finished building all configs. Engage watchcats...\n");

    let event_watcher = Watchcat::new(events::FOLDER);
    let userconfig_watcher = Watchcat::new(userconfigs::FOLDER);

    loop {
        let mut event_changes = event_watcher.get_changed_filenames();
        if !event_changes.is_empty() {
            println!("eventfile change detected... ");
            event_changes.append(&mut event_watcher.get_changed_filenames());
            println!("changed ({:3}): {event_changes:?}", event_changes.len());

            match do_all() {
                Ok(changes) => {
                    _ = write_change_summary(&mut stdout, changes, Changetype::INTERESTING);
                }
                Err(err) => println!("failed to build all {err:#}"),
            }
        }

        for filename in userconfig_watcher.get_changed_filenames() {
            println!("userconfig changed {filename:>16}... ");
            match do_specific(&filename) {
                Ok(change) => println!("{:?} {}", change.changetype, change.name),
                Err(err) => println!("{err:#}"),
            }
        }

        sleep(Duration::from_secs(5));
    }
}

fn do_all() -> anyhow::Result<Vec<Changestatus>> {
    let all = userconfigs::load_all();
    output_files::all_remove_rest(all)
}

fn do_specific(userconfig_filename: &str) -> anyhow::Result<Changestatus> {
    let config = userconfigs::load_specific(userconfig_filename)?;
    output_files::one(config)
}
