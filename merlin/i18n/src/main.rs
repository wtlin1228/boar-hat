use anyhow;
use i18n::{extract_i18n_keys::extract_i18n_keys, GitDiffStatusLetter};
use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::Path,
    process::Command,
};

fn should_track(status_letter: &str, file_path: &str) -> bool {
    GitDiffStatusLetter::should_track(status_letter)
        && (file_path.ends_with(".js")
            || file_path.ends_with(".ts")
            || file_path.ends_with(".jsx")
            || file_path.ends_with(".tsx"))
}

fn main() -> anyhow::Result<()> {
    // 1. get the changed file paths (source files)
    // 2. extract i18n keys from source files
    // 3. append extracted i18n keys to target files (e.g. en.json)
    // 4. (manually) fill in the new keys
    // 5. sort the target files

    let repo_path = "/Users/wtlin1228/iCHEF/Napoleon/napoleon/frontend";
    let mut git = Command::new("git");
    let output = git
        .current_dir(repo_path)
        .arg("diff")
        .arg("--name-status")
        .arg("develop")
        .arg("HEAD")
        .output()
        .expect("git diff failed");

    let mut source_files: Vec<String> = vec![];
    for line in output.stdout.lines() {
        let line = line?;
        let mut iter = line.split("\t");
        let status_letter = &iter.next().expect("get status letter failed")[0..1];
        let file_path = iter.next().expect("get file path failed");
        if should_track(status_letter, file_path) {
            source_files.push(file_path.to_string());
        }
    }

    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let out_file_name = format!("i18n-keys.{}.jsonc", random_string);
    let out_file_path = Path::new(repo_path).join(out_file_name);
    let file = File::create(out_file_path)?;
    let mut stream = BufWriter::new(file);

    stream.write(b"{\n")?;
    for source_file in source_files {
        let extracted_keys = extract_i18n_keys(&format!("{}/{}", repo_path, source_file));
        if extracted_keys.len() > 0 {
            stream.write(format!("\t// {}\n", source_file).as_bytes())?;
        }
        for t in extracted_keys {
            stream.write(t.to_line().as_bytes())?;
        }
    }
    stream.write(b"}\n")?;

    anyhow::Ok(())
}
