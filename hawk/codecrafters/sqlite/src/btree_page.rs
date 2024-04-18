use super::{cell::TableLeafCell, schema_table::SchemaTable};
use anyhow::{bail, Ok, Result};

#[derive(Debug, PartialEq)]
pub enum PageType {
    InteriorIndexBTreePage,
    InteriorTableBTreePage,
    LeafIndexBTreePage,
    LeafTableBTreePage,
}

impl PageType {
    pub fn from(byte: u8) -> Result<Self> {
        match byte {
            0x02 => Ok(Self::InteriorIndexBTreePage),
            0x05 => Ok(Self::InteriorTableBTreePage),
            0x0a => Ok(Self::LeafIndexBTreePage),
            0x0d => Ok(Self::LeafTableBTreePage),
            _ => bail!("Any other value for the b-tree page type is an error"),
        }
    }

    pub fn is_interior_page(&self) -> bool {
        match self {
            PageType::InteriorIndexBTreePage => true,
            PageType::InteriorTableBTreePage => true,
            PageType::LeafIndexBTreePage => false,
            PageType::LeafTableBTreePage => false,
        }
    }
}

/// 1. The 100-byte database file header (found on page 1 only)
/// 2. The 8 or 12 byte b-tree page header
/// 3. The cell pointer array
/// 4. Unallocated space
/// 5. The cell content area
/// 6. The reserved region.
#[derive(Debug)]
pub struct BTreePage {
    pub page_type: PageType,
    pub first_freeblock: u16,
    pub cell_count: u16,
    pub content_area_start_at: u16,
    pub fragmented_free_bytes_count: u8,
    pub right_most_pointer: Option<u32>,
    pub cell_pointers: Vec<u16>,
    pub data: Vec<u8>,
}

impl BTreePage {
    pub fn get_tables(&self) -> Result<Vec<SchemaTable>> {
        assert_eq!(
            self.page_type,
            PageType::LeafTableBTreePage,
            "Can only parse the tables from a leaf table b-tree page now"
        );

        let mut tables = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = match i {
                0 => &self.data[self.cell_pointers[0] as usize..],
                _ => &self.data[self.cell_pointers[i] as usize..self.cell_pointers[i - 1] as usize],
            };
            tables.push(SchemaTable::from(cell)?);
        }
        Ok(tables)
    }

    pub fn get_rows(&self) -> Result<Vec<TableLeafCell>> {
        assert_eq!(
            self.page_type,
            PageType::LeafTableBTreePage,
            "Can only parse the rows from a leaf table b-tree page now"
        );

        let mut rows = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = match i {
                0 => &self.data[self.cell_pointers[0] as usize..],
                _ => &self.data[self.cell_pointers[i] as usize..self.cell_pointers[i - 1] as usize],
            };
            rows.push(TableLeafCell::parse(cell)?);
        }
        Ok(rows)
    }
}
