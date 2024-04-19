mod btree_page;
mod cell;
mod database_file_header;
mod reader_utils;
mod schema_table;
mod serial_value;
pub mod sql_parser;

use anyhow::{Ok, Result};
use btree_page::PageType;
use cell::TableLeafCell;
use database_file_header::DatabaseFileHeader;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::os::unix::fs::FileExt;

use btree_page::BTreePage;
use reader_utils::ReadeInto;
use schema_table::SchemaTable;

pub struct SQLiteDB {
    header: DatabaseFileHeader,
    database_path: String,
    page_size: u16,
}

impl SQLiteDB {
    pub fn new(database_path: &str) -> Result<Self> {
        let mut file = File::open(database_path)?;

        let mut buf = [0; 100];
        file.read_exact(&mut buf)?;
        let database_file_header = DatabaseFileHeader::from_bytes(&buf)?;
        let page_size = u16::from_be_bytes(database_file_header.page_size);

        Ok(Self {
            header: database_file_header,
            database_path: database_path.to_string(),
            page_size,
        })
    }

    pub fn get_page(&self, page: usize) -> Result<BTreePage> {
        let file = File::open(&self.database_path)?;
        let mut buf = vec![0u8; self.page_size as usize];
        file.read_exact_at(&mut buf, (page as u64 - 1) * self.page_size as u64)?;
        let mut reader = match page == 1 {
            true => Cursor::new(&buf[DatabaseFileHeader::DATABASE_FILE_HEADER_SIZE..]),
            false => Cursor::new(&buf[..]),
        };

        let page_type = PageType::from(reader.read_byte()?)?;
        let first_freeblock = reader.read_u16(2)?;
        let cell_count = reader.read_u16(2)?;
        let content_area_start_at = reader.read_u16(2)?;
        let fragmented_free_bytes_count = reader.read_byte()?;
        let right_most_pointer = match page_type.is_interior_page() {
            true => Some(reader.read_u32(4)?),
            false => None,
        };
        let mut cell_pointers = Vec::with_capacity(cell_count as usize);
        for _ in 0..cell_count {
            cell_pointers.push(reader.read_u16(2)?);
        }

        Ok(BTreePage {
            page_type,
            first_freeblock,
            cell_count,
            content_area_start_at,
            fragmented_free_bytes_count,
            right_most_pointer,
            cell_pointers,
            data: buf,
        })
    }

    pub fn get_tables(&self) -> Result<Vec<SchemaTable>> {
        let page1 = self.get_page(1)?;
        page1.get_tables()
    }

    pub fn get_page_size(&self) -> u16 {
        self.page_size
    }

    pub fn get_table_rows(&self, table: &SchemaTable) -> Result<Vec<TableLeafCell>> {
        assert!(
            table.rootpage.is_some(),
            "Can't get rows from a table without rootpage"
        );
        let mut rows = vec![];
        let mut pages = vec![table.rootpage.unwrap()];
        while pages.len() > 0 {
            let mut next_pages = vec![];
            for page in pages {
                let btree_page = self.get_page(page)?;
                match btree_page.page_type {
                    PageType::InteriorTableBTreePage => {
                        next_pages.append(&mut btree_page.get_child_pages()?);
                    }
                    PageType::LeafTableBTreePage => {
                        rows.append(&mut btree_page.get_rows()?);
                    }
                    PageType::LeafIndexBTreePage => unreachable!(),
                    PageType::InteriorIndexBTreePage => unreachable!(),
                }
            }
            pages = next_pages;
        }

        Ok(rows)
    }
}
