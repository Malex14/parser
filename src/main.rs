use crate::build::build_all_remove_rest;
use crate::changestatus::create_change_summary;

mod apply_changes;
mod build;
mod changestatus;
mod events;
mod generate_ics;
mod userconfig;
mod userconfigs;

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
}
