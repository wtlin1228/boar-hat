#![warn(missing_docs, missing_debug_implementations)]

//! # JSON Sorter Library
//!
//! The JSON Sorter library provides functionality to read a JSON file, sort its contents by keys,
//! and write the sorted JSON back to a file.
//!
//! ## Usage
//!
//! ```rust no_run
//! use i18n_json_sorter::JsonSorter;
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
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

use anyhow::Context;

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
    pub fn from(file_path: &str) -> anyhow::Result<Self> {
        let mut json_sorter = JsonSorter { contents: vec![] };
        let mut file = File::open(file_path)?;
        let reader = BufReader::new(&mut file);
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            match line {
                s if s.is_empty() || s == "{" || s == "}" || s == "{}" => (),
                _ => {
                    let (key, value) = line.split_once(":").context("colon expected")?;
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
    pub fn sort_contents(&mut self) -> anyhow::Result<()> {
        self.contents.sort_by(|a, b| {
            let a = &a.key[..];
            let b = &b.key[..];
            a.cmp(b)
        });
        Ok(())
    }

    /// Writes the contents of the JSON Sorter to a file located at the given `file_path`.
    pub fn write_to_file(&self, file_path: &str) -> anyhow::Result<()> {
        let _ = fs::remove_file(file_path);
        let file = File::create(file_path)?;
        let mut stream = BufWriter::new(file);

        stream.write(b"{\n")?;
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
            stream.write(line.as_bytes())?;
        }
        stream.write(b"}\n")?;

        Ok(())
    }
}
