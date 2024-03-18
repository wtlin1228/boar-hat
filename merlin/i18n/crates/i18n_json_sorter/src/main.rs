use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use std::path::Path;

use i18n_json_sorter::JsonSorter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Absolute path to apply json sorter, either directory or file
    path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let path = Path::new(&cli.path[..]);

    let mut target_to_sort: Vec<String> = vec![];
    match path.is_dir() {
        true => {
            for entry in path.read_dir()? {
                if let Ok(entry) = entry {
                    let path = Path::join(path, entry.path());
                    target_to_sort.push(
                        path.to_str()
                            .with_context(|| format!("convert path {:?} to str failed", path))?
                            .to_string(),
                    );
                }
            }
        }
        false => target_to_sort.push(
            path.to_str()
                .with_context(|| format!("convert path {:?} to str failed", path))?
                .to_string(),
        ),
    };

    for target in target_to_sort.iter() {
        match sort_single_file(&target[..]) {
            Ok(_) => {
                println!("{} {}", "Done".bold().bright_green(), target);
            }
            Err(_) => {
                println!("{} {}", "Fail".bold().bright_red(), target);
            }
        }
    }

    Ok(())
}

fn sort_single_file(path: &str) -> anyhow::Result<()> {
    let mut json_sorter = JsonSorter::from(path)?;
    json_sorter.sort_contents()?;
    json_sorter.write_to_file(path)?;

    Ok(())
}
