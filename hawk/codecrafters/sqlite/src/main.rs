use anyhow::{bail, Result};
use sqlite_starter_rust::SQLiteDB;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let db = SQLiteDB::new(&args[1])?;
            println!("database page size: {}", db.get_page_size());
            println!("number of tables: {}", db.get_tables()?.len());
        }
        ".tables" => {
            let db = SQLiteDB::new(&args[1])?;
            for name in db.get_tables()?.iter().map(|table| table.get_tbl_name()) {
                // ref: https://www.sqlite.org/fileformat2.html#internal_schema_objects
                if !name.starts_with("sqlite_") {
                    print!("{} ", name);
                }
            }
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
