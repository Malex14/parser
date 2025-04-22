use std::fs;
use std::path::Path;

use anyhow::Context as _;

use crate::userconfig::UserconfigFile;

pub const FOLDER: &str = "userconfig";

pub fn load_specific(filename: &str) -> anyhow::Result<UserconfigFile> {
    let path = Path::new(FOLDER).join(filename);
    let content = fs::read_to_string(path).context("failed to read")?;
    let parsed: UserconfigFile = serde_json::from_str(&content).context("failed to parse")?;
    Ok(parsed)
}

pub fn load_all() -> Vec<UserconfigFile> {
    let mut successful: Vec<UserconfigFile> = Vec::new();

    let existing_files = get_existing_files().expect("should be able to read userconfig directory");

    for filename in existing_files {
        match load_specific(&filename) {
            Ok(content) => successful.push(content),
            Err(err) => println!("skip userconfig {filename:>16}: {err:#}"),
        }
    }

    successful
}

fn get_existing_files() -> std::io::Result<Vec<String>> {
    let mut list: Vec<String> = Vec::new();
    for maybe_entry in fs::read_dir(FOLDER)? {
        let filename = maybe_entry?
            .file_name()
            .into_string()
            .expect("filename should be UTF8");

        #[expect(clippy::case_sensitive_file_extension_comparisons)]
        if filename.ends_with(".json") {
            list.push(filename);
        }
    }

    Ok(list)
}
