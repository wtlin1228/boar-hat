use anyhow;
use std::process::Command;

enum GitDiffStatusLetter {
    A, // addition of a file
    C, // copy of a file into a new one
    D, // deletion of a file
    M, // modification of the contents or mode of a file
    R, // renaming of a file
    T, // change in the type of the file (regular file, symbolic link or submodule)
    U, // file is unmerged (you must complete the merge before it can be committed)
    X, // "unknown" change type (most probably a bug, please report it)
}

fn main() -> anyhow::Result<()> {
    // 1. get the changed file paths (source files)
    // 2. extract i18n keys from source files
    // 3. append extracted i18n keys to target files (e.g. en.json)
    // 4. (manually) fill in the new keys
    // 5. sort the target files

    let repo_path = "/Users/leo/iCHEF/Napoleon/napoleon/frontend";
    let mut git = Command::new("git");
    let output = git
        .current_dir(repo_path)
        .arg("diff")
        .arg("--name-status")
        .arg("develop")
        .arg("HEAD")
        .output()
        .expect("git diff failed");
    println!("{:?}", output);

    anyhow::Ok(())
}
