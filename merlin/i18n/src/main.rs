use anyhow;
use clap::Parser;
use i18n::{extract_i18n_keys::extract_i18n_keys, GitDiffStatusLetter};
use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::Path,
    process::Command,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Absolute path for your repository
    #[arg(short, long)]
    repo_path: String,

    /// The first commit for git diff <commit> <commit>
    #[arg(short, long)]
    commit1: String,

    /// The second commit for git diff <commit> <commit>
    #[arg(short = 'd', long, default_value_t = String::from("HEAD"))]
    commit2: String,

    /// Include the file names where keys extracted from
    #[arg(short, long, default_value_t = false)]
    include_file_name: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let repo_path = cli.repo_path;
    let commit1 = cli.commit1;
    let commit2 = cli.commit2;
    let should_output_file_name = cli.include_file_name;

    // git diff --name-status <commit> <commit>
    let mut git = Command::new("git");
    let output = git
        .current_dir(&repo_path)
        .arg("diff")
        .arg("--name-status")
        .arg(commit1)
        .arg(commit2)
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

    let out_file_name = generate_output_file_name();
    let out_file_path = Path::new(&repo_path).join(out_file_name);
    let file = File::create(out_file_path)?;
    let mut stream = BufWriter::new(file);

    stream.write(b"{\n")?;
    for source_file in source_files {
        let extracted_keys = extract_i18n_keys(&format!("{}/{}", repo_path, source_file));
        if extracted_keys.len() > 0 && should_output_file_name {
            stream.write(format!("\t// {}\n", source_file).as_bytes())?;
        }
        for t in extracted_keys {
            stream.write(t.to_line().as_bytes())?;
        }
    }
    stream.write(b"}\n")?;

    anyhow::Ok(())
}

fn should_track(status_letter: &str, file_path: &str) -> bool {
    GitDiffStatusLetter::should_track(status_letter)
        && (file_path.ends_with(".js")
            || file_path.ends_with(".ts")
            || file_path.ends_with(".jsx")
            || file_path.ends_with(".tsx"))
}

fn generate_output_file_name() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    format!("i18n-keys.{}.jsonc", random_string)
}
