mod btree_page;
pub mod cell;
mod database_file_header;
mod reader_utils;
mod schema_table;
mod serial_value;
pub mod sql_parser;

use anyhow::{bail, Ok, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::os::unix::fs::FileExt;

use crate::{
    btree_page::{BTreePage, PageType},
    cell::{TableInteriorCell, TableLeafCell},
    database_file_header::DatabaseFileHeader,
    reader_utils::ReadeInto,
    schema_table::SchemaTable,
    sql_parser::{CreateIndexStmt, WhereClause},
};

pub struct SQLiteDB {
    #[allow(unused)]
    header: DatabaseFileHeader,
    database_path: String,
    page_size: u16,
    tables: Vec<SchemaTable>,
}

impl SQLiteDB {
    pub fn new(database_path: &str) -> Result<Self> {
        let mut file = File::open(database_path)?;

        let mut buf = [0; 100];
        file.read_exact(&mut buf)?;
        let database_file_header = DatabaseFileHeader::from_bytes(&buf)?;
        let page_size = u16::from_be_bytes(database_file_header.page_size);

        let mut db = Self {
            header: database_file_header,
            database_path: database_path.to_string(),
            page_size,
            tables: vec![],
        };
        let page1 = db.get_page(1)?;
        db.tables = page1.get_tables()?;
        Ok(db)
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

    pub fn get_tables(&self) -> &[SchemaTable] {
        &self.tables
    }

    pub fn get_table(&self, table_name: &str) -> Option<&SchemaTable> {
        self.tables
            .iter()
            .filter(|&x| x.object_type.is_table())
            .find(|&x| &x.tbl_name == &table_name)
    }

    pub fn get_index(&self, table_name: &str, column: &str) -> Option<&SchemaTable> {
        self.tables.iter().find(|&x| match &x.object_type {
            schema_table::ObjectType::Index(CreateIndexStmt {
                on, indexed_column, ..
            }) => on == table_name && indexed_column[0] == column,
            _ => false,
        })
    }

    pub fn get_page_size(&self) -> u16 {
        self.page_size
    }

    fn get_row(&self, table: &SchemaTable, row_id: u64) -> Result<TableLeafCell> {
        let mut btree_page = self.get_page(table.rootpage.unwrap())?;
        'outer: while btree_page.page_type.is_interior_page() {
            for i in 0..btree_page.cell_pointers.len() {
                let cell = TableInteriorCell::parse(
                    &btree_page.data[btree_page.cell_pointers[i] as usize..],
                )?;
                if cell.row_id > row_id {
                    btree_page = self.get_page(cell.page_number_of_left_child)?;
                    continue 'outer;
                }
            }
            btree_page = self.get_page(btree_page.right_most_pointer.unwrap() as usize)?;
        }

        assert!(!btree_page.page_type.is_interior_page());

        for i in 0..btree_page.cell_pointers.len() {
            let cell =
                TableLeafCell::parse(&btree_page.data[btree_page.cell_pointers[i] as usize..])?;
            if cell.row_id == row_id {
                return Ok(cell);
            }
        }

        bail!("Row with row_id {} not exists", row_id)
    }

    pub fn get_table_rows(
        &self,
        table: &SchemaTable,
        where_clause: Option<&WhereClause>,
    ) -> Result<Vec<TableLeafCell>> {
        assert!(
            table.rootpage.is_some(),
            "Can't get rows from a table without rootpage"
        );

        let column_def = table.get_table_column_def()?;
        let row_filter = match where_clause {
            Some(WhereClause { column, value }) => {
                let column_idx = match column_def.iter().position(|x| x == column) {
                    Some(column_idx) => column_idx,
                    None => bail!("Where condition column {} doesn't exist", column),
                };
                Some(move |row: &TableLeafCell| {
                    format!("{}", row.columns[column_idx]) == value.as_str()
                })
            }
            None => None,
        };

        let mut pages = match where_clause {
            Some(WhereClause { column, .. }) => {
                match self.get_index(table.tbl_name.as_ref(), column) {
                    Some(index) => vec![index.rootpage.unwrap()],
                    None => vec![table.rootpage.unwrap()],
                }
            }
            None => vec![table.rootpage.unwrap()],
        };

        let mut rows: Vec<TableLeafCell> = vec![];
        while pages.len() > 0 {
            let mut next_pages = vec![];
            for page in pages {
                let btree_page = self.get_page(page)?;
                match btree_page.page_type {
                    PageType::InteriorTableBTreePage => {
                        next_pages.append(&mut btree_page.get_table_child_pages()?);
                    }
                    PageType::LeafTableBTreePage => {
                        rows.append(&mut btree_page.get_table_rows(row_filter.as_ref())?)
                    }
                    PageType::LeafIndexBTreePage => {
                        let where_value = where_clause.unwrap().value.as_str();
                        for row in btree_page.get_index_rows(where_value)? {
                            // convert IndexLeafCell to TableLeafCell
                            rows.push(self.get_row(table, row.row_id)?);
                        }
                    }
                    PageType::InteriorIndexBTreePage => {
                        let where_value = where_clause.unwrap().value.as_str();
                        next_pages.append(&mut btree_page.get_index_child_page(where_value)?);
                    }
                }
            }
            pages = next_pages;
        }
        Ok(rows)
    }
}
