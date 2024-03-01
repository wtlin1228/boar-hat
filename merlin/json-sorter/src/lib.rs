#![warn(missing_docs, missing_debug_implementations)]

//! # JSON Sorter Library
//!
//! The JSON Sorter library provides functionality to read a JSON file, sort its contents by keys,
//! and write the sorted JSON back to a file.
//!
//! ## Usage
//!
//! ```rust no_run
//! use json_sorter::JsonSorter;
//!
//! fn main() {
//!     // Example usage
//!     let file_path = "path/to/your/json/file.json";
//!     let mut sorter = JsonSorter::from(file_path).unwrap();
//!     sorter.sort_contents().unwrap();
//!     sorter.write_to_file("sorted_file.json").unwrap();
//! }
//! ```

use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

/// Alias for the error type returned by JsonSorter methods.
type JsonSorterError = Box<dyn Error + Send + Sync + 'static>;

/// Represents a key-value pair in the JSON Sorter.
#[derive(Debug)]
struct KeyValuePair {
    key: String,
    value: String,
}

/// Represents a JSON Sorter instance.
#[derive(Debug)]
pub struct JsonSorter {
    contents: Vec<KeyValuePair>,
}

impl JsonSorter {
    /// Constructs a new JsonSorter instance from the JSON file located at the given `file_path`.
    pub fn from(file_path: &str) -> Result<Self, JsonSorterError> {
        let mut json_sorter = JsonSorter { contents: vec![] };
        let mut file = File::open(file_path)?;
        let reader = BufReader::new(&mut file);
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            match line {
                s if s.is_empty() || s == "{" || s == "}" || s == "{}" => (),
                _ => {
                    let (key, value) = line.split_once(":").expect("colon expected");
                    let value = value.trim();
                    json_sorter.contents.push(KeyValuePair {
                        key: key[1..key.len() - 1].to_owned(), // "key" => key
                        value: match &value[value.len() - 1..] {
                            "," => value[1..value.len() - 2].to_owned(), // "value", => value
                            _ => value[1..value.len() - 1].to_owned(),   // "value" => value
                        },
                    });
                }
            }
        }
        Ok(json_sorter)
    }

    /// Sorts the contents of the JSON Sorter by keys.
    pub fn sort_contents(&mut self) -> Result<(), JsonSorterError> {
        self.contents.sort_by(|a, b| {
            let a = &a.key[..];
            let b = &b.key[..];
            a.cmp(b)
        });
        Ok(())
    }

    /// Writes the contents of the JSON Sorter to a file located at the given `file_path`.
    pub fn write_to_file(&self, file_path: &str) -> Result<(), JsonSorterError> {
        let _ = fs::remove_file(file_path);
        let mut file = File::create_new(file_path)?;
        file.write_all(b"{\n")?;
        let mut lines = vec![];
        for (i, KeyValuePair { key, value }) in self.contents.iter().enumerate() {
            let line = format!(
                "    \"{}\": \"{}\"{}",
                key,
                value,
                if i != self.contents.len() - 1 {
                    ",\n"
                } else {
                    "\n"
                }
            );
            lines.push(line);
            if lines.len() > 1_000_000 {
                file.write_all(lines.join("").as_bytes())?;
                lines.clear();
            }
        }
        if lines.len() > 0 {
            file.write_all(lines.join("").as_bytes())?;
        }
        file.write_all(b"}\n")?;
        Ok(())
    }
}
