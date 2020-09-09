mod apply_changes;
mod build;
mod events;
mod generate_ics;
mod userconfig;
mod userconfigs;

fn main() {
    build::ensure_directory().unwrap();
    println!("Hello, world!");
}
