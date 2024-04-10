use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::FileExt;

use sqlite_starter_rust::database_file_header::DatabaseFileHeader;
use sqlite_starter_rust::{BTreePage, SchemaTable};

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
            let mut file = File::open(&args[1])?;
            let mut buff = [0; 100];
            file.read_exact(&mut buff)?;
            let database_file_header = DatabaseFileHeader::from_bytes(&buff)?;
            let page_size = u16::from_be_bytes(database_file_header.page_size);
            println!("database page size: {}", page_size);

            let mut buff = vec![0u8; page_size as usize];
            file.read_exact_at(&mut buff, 0)?;
            let page = BTreePage::new(&buff, Some(database_file_header))?;
            println!("number of tables: {}", page.cell_count);
        }
        ".tables" => {
            let mut file = File::open(&args[1])?;
            let mut buff = [0; 100];
            file.read_exact(&mut buff)?;
            let database_file_header = DatabaseFileHeader::from_bytes(&buff)?;
            let page_size = u16::from_be_bytes(database_file_header.page_size);
            println!("database page size: {}", page_size);

            let mut buff = vec![0u8; page_size as usize];
            file.read_exact_at(&mut buff, 0)?;
            let page = BTreePage::new(&buff, Some(database_file_header))?;

            let mut tbl_names = vec![];
            for i in 0..page.cell_pointers.len() {
                let cell = match i {
                    0 => &buff[page.cell_pointers[0] as usize..],
                    _ => &buff[page.cell_pointers[i] as usize..page.cell_pointers[i - 1] as usize],
                };
                let schema_table = SchemaTable::from(cell)?;

                // ref: https://www.sqlite.org/fileformat2.html#internal_schema_objects
                if !schema_table.tbl_name.starts_with("sqlite_") {
                    tbl_names.push(schema_table.tbl_name);
                }
            }
            println!("{:?}", tbl_names.join(" "));
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
