use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Changetype {
    Added,
    Changed,
    Moved,
    Removed,
    Same,
    Skipped,
}

impl Changetype {
    pub const ALL: &'static [Self] = &[
        Self::Added,
        Self::Changed,
        Self::Moved,
        Self::Removed,
        Self::Same,
        Self::Skipped,
    ];
    pub const INTERESTING: &'static [Self] =
        &[Self::Added, Self::Changed, Self::Moved, Self::Removed];

    const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Changed => "changed",
            Self::Moved => "moved",
            Self::Removed => "removed",
            Self::Same => "same",
            Self::Skipped => "skipped",
        }
    }
}

pub struct Changestatus {
    pub name: String,
    pub changetype: Changetype,
}

pub fn create_change_summary(changes: Vec<Changestatus>, to_be_shown: &[Changetype]) -> String {
    let mut map: HashMap<Changetype, Vec<String>> = HashMap::new();

    for change in changes {
        map.entry(change.changetype).or_default().push(change.name);
    }

    let mut lines: Vec<String> = Vec::new();
    for key in to_be_shown {
        if let Some(val) = map.get_mut(key) {
            val.sort_by_key(|o| o.to_lowercase());
            let key = key.as_str();
            lines.push(format!("{key:7} ({:3}): {val:?}", val.len()));
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
    let result = create_change_summary(vec![], Changetype::ALL);
    assert_eq!(result, "");
}

#[test]
fn summary_shows_every_type_once() {
    let result = create_change_summary(generate_every_type_once(), Changetype::ALL);
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
    let result = create_change_summary(generate_every_type_once(), Changetype::INTERESTING);
    assert_eq!(
        result,
        r#"added   (  1): ["A"]
changed (  1): ["C"]
moved   (  1): ["M"]
removed (  1): ["R"]"#
    );
}
