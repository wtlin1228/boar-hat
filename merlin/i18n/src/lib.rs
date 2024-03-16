#[derive(Debug, PartialEq)]
pub struct SingleTranslation {
    pub key: String,
    pub value: String,
}

impl SingleTranslation {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn to_line(&self) -> String {
        format!("\t\"{}\": \"\",\n", self.value)
    }
}

pub struct GitDiffStatusLetter;

/// Possible status letters are:
/// - A: addition of a file
/// - C: copy of a file into a new one
/// - D: deletion of a file
/// - M: modification of the contents or mode of a file
/// - R: renaming of a file
/// - T: change in the type of the file (regular file, symbolic link or submodule)
/// - U: file is unmerged (you must complete the merge before it can be committed)
/// - X: "unknown" change type (most probably a bug, please report it)
impl GitDiffStatusLetter {
    pub fn should_track(s: &str) -> bool {
        match s {
            "A" => true,
            "C" => true,
            "D" => false,
            "M" => true,
            "R" => false,
            "T" => false,
            "U" => false,
            "X" => false,
            _ => unreachable!(),
        }
    }
}

pub mod extract_i18n_keys;
pub mod i18n_key_visitor;
