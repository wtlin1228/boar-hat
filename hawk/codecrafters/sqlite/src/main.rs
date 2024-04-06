use anyhow::{bail, Result};
use std::fs::File;
use std::io::prelude::*;

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct DatabaseHeader {
    header_string: [u8; 16],
    page_size: [u8; 2],
    file_format_write_version: [u8; 1],
    file_format_read_version: [u8; 1],
    reserved_space_at_the_end_of_each_page: [u8; 1],
    maximum_embedded_payload_fraction: [u8; 1],
    minimum_embedded_payload_fraction: [u8; 1],
    leaf_payload_fraction: [u8; 1],
    file_change_counter: [u8; 4],
    size_of_the_database_file_in_pages: [u8; 4],
    page_number_of_the_first_freelist_trunk_page: [u8; 4],
    total_number_of_freelist_pages: [u8; 4],
    the_schema_cookie: [u8; 4],
    the_schema_format_number: [u8; 4],
    default_page_cache_size: [u8; 4],
    the_page_number_of_the_largest_root_b_tree_page: [u8; 4],
    the_database_text_encoding: [u8; 4],
    user_version: [u8; 4],
    incremental_vacuum_mode: [u8; 4],
    application_id: [u8; 4],
    reserved_for_expansion: [u8; 20],
    version_valid_for_number: [u8; 4],
    sqlite_version_number: [u8; 4],
}

impl DatabaseHeader {
    const DATABASE_HEADER_SIZE: usize = std::mem::size_of::<DatabaseHeader>();
    fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() != Self::DATABASE_HEADER_SIZE {
            bail!("fail construct database header from input data due to length mismatch");
        }

        let d = data as *const [u8] as *const DatabaseHeader;
        Ok(unsafe { *d.clone() })
    }
}

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
            let database_header = DatabaseHeader::from_bytes(&buff)?;

            println!(
                "database page size: {}",
                u16::from_be_bytes(database_header.page_size)
            );
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
