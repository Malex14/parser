use std::collections::HashMap;

#[derive(PartialEq, Debug, Hash)]
pub enum Changetype {
    Added,
    Changed,
    Moved,
    Removed,
    Same,
    Skipped,
}

#[derive(Debug)]
pub struct Changestatus {
    pub name: String,
    pub changetype: Changetype,
}

pub const SHOW_ALL: [&str; 6] = ["added", "changed", "moved", "removed", "same", "skipped"];
pub const SHOW_INTERESTING: [&str; 4] = ["added", "changed", "moved", "removed"];

pub fn create_change_summary(changes: &[Changestatus], to_be_shown: &[&str]) -> String {
    let mut map: HashMap<&str, Vec<String>> = HashMap::new();

    for change in changes {
        let key = match change.changetype {
            Changetype::Added => "added",
            Changetype::Changed => "changed",
            Changetype::Moved => "moved",
            Changetype::Removed => "removed",
            Changetype::Same => "same",
            Changetype::Skipped => "skipped",
        };

        if !map.contains_key(key) {
            map.insert(key, Vec::new());
        }

        let vec = map.get_mut(key).unwrap();
        vec.push(change.name.to_owned());
    }

    let mut lines: Vec<String> = Vec::new();
    for key in to_be_shown {
        if let Some(val) = map.get_mut(key) {
            val.sort_by_key(|o| o.to_lowercase());
            lines.push(format!("{:7} ({:3}): {:?}", key, val.len(), val));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
fn generate_every_type_once() -> Vec<Changestatus> {
    vec![
        Changestatus {
            name: String::from("A"),
            changetype: Changetype::Added,
        },
        Changestatus {
            name: String::from("C"),
            changetype: Changetype::Changed,
        },
        Changestatus {
            name: String::from("M"),
            changetype: Changetype::Moved,
        },
        Changestatus {
            name: String::from("R"),
            changetype: Changetype::Removed,
        },
        Changestatus {
            name: String::from("Sa"),
            changetype: Changetype::Same,
        },
        Changestatus {
            name: String::from("Sk"),
            changetype: Changetype::Skipped,
        },
    ]
}

#[test]
fn summary_without_changes_is_empty() {
    let result = create_change_summary(&[], &SHOW_ALL);
    assert_eq!(result, "");
}

#[test]
fn summary_shows_every_type_once() {
    let result = create_change_summary(&generate_every_type_once(), &SHOW_ALL);
    assert_eq!(
        result,
        r#"added   (  1): ["A"]
changed (  1): ["C"]
moved   (  1): ["M"]
removed (  1): ["R"]
same    (  1): ["Sa"]
skipped (  1): ["Sk"]"#
    );
}

#[test]
fn summary_shows_interesting_types_once() {
    let result = create_change_summary(&generate_every_type_once(), &SHOW_INTERESTING);
    assert_eq!(
        result,
        r#"added   (  1): ["A"]
changed (  1): ["C"]
moved   (  1): ["M"]
removed (  1): ["R"]"#
    );
}
