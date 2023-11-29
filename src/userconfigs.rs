use std::fs;
use std::path::Path;

use crate::userconfig::UserconfigFile;

pub const FOLDER: &str = "userconfig";

pub fn load_specific(filename: &str) -> Result<UserconfigFile, String> {
    let path = Path::new(FOLDER).join(filename);
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read: {err}"))?;
    let parsed: UserconfigFile =
        serde_json::from_str(&content).map_err(|err| format!("failed to parse: {err}"))?;

    Ok(parsed)
}

pub fn load_all() -> Vec<UserconfigFile> {
    let mut successful: Vec<UserconfigFile> = Vec::new();

    let existing_files = get_existing_files().expect("should be able to read userconfig directory");

    for filename in existing_files {
        match load_specific(&filename) {
            Ok(content) => successful.push(content),
            Err(err) => println!("skip userconfig {filename:>16}: {err}"),
        }
    }

    successful
}

fn get_existing_files() -> Result<Vec<String>, std::io::Error> {
    let mut list: Vec<String> = Vec::new();
    for maybe_entry in fs::read_dir(FOLDER)? {
        let filename = maybe_entry?
            .file_name()
            .into_string()
            .expect("filename should be UTF8");

        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        if filename.ends_with(".json") {
            list.push(filename);
        }
    }

    Ok(list)
}
