use std::{fs, io::BufRead};

use json_sorter::JsonSorter;

#[test]
fn sort_json_file() {
    let mut json_sorter = JsonSorter::from("./tests/inputs/good.json").unwrap();
    json_sorter.sort_contents().unwrap();
    json_sorter
        .write_to_file("./tests/outputs/good.json")
        .unwrap();

    let output_file = fs::read("./tests/outputs/good.json").unwrap();
    let lines: Vec<String> = output_file.lines().map(|l| l.unwrap()).collect();
    assert_eq!(
        lines,
        [
            "{",
            "    \"foo\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.a\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.b\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.baz.a\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.baz.b\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.baz.c\": \"Lorem ipsum dolor sit amet.\",",
            "    \"foo.bar.c\": \"Lorem ipsum dolor sit amet.\"",
            "}",
        ]
    );
}

#[test]
fn sort_empty_json_file() {
    let mut json_sorter = JsonSorter::from("./tests/inputs/good.empty.json").unwrap();
    json_sorter.sort_contents().unwrap();
    json_sorter
        .write_to_file("./tests/outputs/good.empty.json")
        .unwrap();

    let output_file = fs::read("./tests/outputs/good.empty.json").unwrap();
    let lines: Vec<String> = output_file.lines().map(|l| l.unwrap()).collect();
    assert_eq!(lines, ["{", "}"]);
}

#[test]
#[should_panic(expected = "colon expected")]
fn sort_bad_json_file() {
    let _ = JsonSorter::from("./tests/inputs/bad.json");
}
