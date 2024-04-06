pub mod database_file_header;

use anyhow::{bail, Result};
use database_file_header::DatabaseFileHeader;

#[derive(Debug)]
pub enum PageType {
    InteriorIndexBTreePage,
    InteriorTableBTreePage,
    LeafIndexBTreePage,
    LeafTableBTreePage,
}

impl PageType {
    fn from(byte: u8) -> Result<Self> {
        match byte {
            0x02 => Ok(Self::InteriorIndexBTreePage),
            0x05 => Ok(Self::InteriorTableBTreePage),
            0x0a => Ok(Self::LeafIndexBTreePage),
            0x0d => Ok(Self::LeafTableBTreePage),
            _ => bail!("Any other value for the b-tree page type is an error"),
        }
    }

    fn is_interior_page(&self) -> bool {
        match self {
            PageType::InteriorIndexBTreePage => true,
            PageType::InteriorTableBTreePage => true,
            PageType::LeafIndexBTreePage => false,
            PageType::LeafTableBTreePage => false,
        }
    }
}

// 1. The 100-byte database file header (found on page 1 only)
// 2. The 8 or 12 byte b-tree page header
// 3. The cell pointer array
// 4. Unallocated space
// 5. The cell content area
// 6. The reserved region.
pub struct BTreePage {
    database_file_header: Option<DatabaseFileHeader>,

    page_type: PageType,
    first_freeblock: u16,
    pub cell_count: u16,
    content_area_start_at: u16,
    fragmented_free_bytes_count: u8,
    right_most_pointer: Option<u32>,
    cell_pointers: Vec<u16>,
}

impl BTreePage {
    pub fn new(data: &[u8], database_file_header: Option<DatabaseFileHeader>) -> Result<Self> {
        let data = match database_file_header {
            Some(_) => &data[DatabaseFileHeader::DATABASE_FILE_HEADER_SIZE..],
            None => data,
        };

        let mut offset = 0;
        let mut next_byte = move || {
            offset += 1;
            data[offset - 1]
        };

        let page_type = PageType::from(next_byte())?;
        let first_freeblock = u16::from_be_bytes([next_byte(), next_byte()]);
        let cell_count = u16::from_be_bytes([next_byte(), next_byte()]);
        let content_area_start_at = u16::from_be_bytes([next_byte(), next_byte()]);
        let fragmented_free_bytes_count = next_byte();
        let right_most_pointer = match page_type.is_interior_page() {
            true => Some(u32::from_be_bytes([
                next_byte(),
                next_byte(),
                next_byte(),
                next_byte(),
            ])),
            false => None,
        };
        let mut cell_pointers = Vec::with_capacity(cell_count as usize);
        for _ in 0..cell_count {
            cell_pointers.push(u16::from_be_bytes([next_byte(), next_byte()]));
        }

        Ok(Self {
            database_file_header,
            page_type,
            first_freeblock,
            cell_count,
            content_area_start_at,
            fragmented_free_bytes_count,
            right_most_pointer,
            cell_pointers,
        })
    }
}

#[derive(Debug)]
enum ObjectType {
    Table,
    Index,
    View,
    Trigger,
}

// ref: https://www.sqlite.org/schematab.html
#[derive(Debug)]
struct SchemaTable {
    object_type: ObjectType,
    name: String,
    tbl_name: String,
    rootpage: Option<usize>,
    sql: String,
}

impl SchemaTable {}
