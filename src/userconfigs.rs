use crate::userconfig::UserconfigFile;
use std::fs;
use std::path::Path;

const FOLDER: &str = "userconfig";

pub fn load_specific(filename: &str) -> Result<UserconfigFile, String> {
    let path = Path::new(FOLDER).join(filename);
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read: {}", err))?;
    let parsed: UserconfigFile =
        serde_json::from_str(&content).map_err(|err| format!("failed to parse: {}", err))?;

    Ok(parsed)
}

pub fn load_all() -> Result<Vec<UserconfigFile>, String> {
    let mut successful: Vec<UserconfigFile> = Vec::new();

    let existing_files = get_existing_files()
        .map_err(|err| format!("failed to get existing userconfig files from dir {}", err))?;

    for filename in existing_files {
        match load_specific(&filename) {
            Ok(content) => successful.push(content),
            Err(err) => println!("skip userconfig {:>16}: {}", filename, err),
        }
    }

    Ok(successful)
}

fn get_existing_files() -> Result<Vec<String>, std::io::Error> {
    let mut list: Vec<String> = Vec::new();
    for maybe_entry in fs::read_dir(FOLDER)? {
        let filename = maybe_entry?
            .file_name()
            .into_string()
            .expect("filename contains something that can not be read easily with rust");

        if filename.ends_with(".json") {
            list.push(filename.to_owned());
        }
    }

    Ok(list)
}
