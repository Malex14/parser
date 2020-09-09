use crate::userconfig::UserconfigFile;
use std::fs;
use std::path::Path;

const FOLDER: &str = "userconfig";

pub fn load_specific(filename: &str) -> Result<UserconfigFile, String> {
    let path = Path::new(FOLDER).join(filename);
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read event file {} Error: {}", filename, err))?;
    let parsed: UserconfigFile = serde_json::from_str(&content)
        .map_err(|err| format!("failed to parse event file {} Error: {}", filename, err))?;

    Ok(parsed)
}
