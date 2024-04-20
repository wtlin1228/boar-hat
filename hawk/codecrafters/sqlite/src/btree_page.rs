use super::{
    cell::{IndexInteriorCell, IndexLeafCell, TableInteriorCell, TableLeafCell},
    schema_table::SchemaTable,
};
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
            let cell = &self.data[self.cell_pointers[i] as usize..];
            tables.push(SchemaTable::from(cell)?);
        }
        Ok(tables)
    }

    pub fn get_table_rows(
        &self,
        row_filter: Option<&impl Fn(&TableLeafCell) -> bool>,
    ) -> Result<Vec<TableLeafCell>> {
        assert_eq!(
            self.page_type,
            PageType::LeafTableBTreePage,
            "Can only parse the rows from a leaf table b-tree page now"
        );

        let mut rows: Vec<TableLeafCell> = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = &self.data[self.cell_pointers[i] as usize..];
            let row = TableLeafCell::parse(cell)?;
            match row_filter {
                Some(f) => match f(&row) {
                    true => rows.push(row),
                    false => (),
                },
                None => rows.push(row),
            }
        }
        Ok(rows)
    }

    pub fn get_table_child_pages(&self) -> Result<Vec<usize>> {
        assert_eq!(
            self.page_type,
            PageType::InteriorTableBTreePage,
            "Can only parse the child pages from a interior table b-tree page now"
        );

        let mut rows: Vec<usize> = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = &self.data[self.cell_pointers[i] as usize..];
            rows.push(TableInteriorCell::parse(cell)?.page_number_of_left_child);
        }
        rows.push(self.right_most_pointer.unwrap() as usize);
        Ok(rows)
    }

    pub fn get_index_rows(&self, where_value: &str) -> Result<Vec<IndexLeafCell>> {
        assert_eq!(self.page_type, PageType::LeafIndexBTreePage);

        // Could use binary search here, but since the disk read dominates so let's just keep it simple.
        let mut rows: Vec<IndexLeafCell> = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = &self.data[self.cell_pointers[i] as usize..];
            let row = IndexLeafCell::parse(cell)?;
            if row.get_first_column_value() == where_value {
                rows.push(row);
            }
        }

        Ok(rows)
    }

    pub fn get_index_child_page(&self, where_value: &str) -> Result<Vec<usize>> {
        assert_eq!(self.page_type, PageType::InteriorIndexBTreePage);

        // Could use binary search here, but since the disk read dominates so let's just keep it simple.
        let mut parsed_cells: Vec<IndexInteriorCell> = vec![];
        for i in 0..self.cell_pointers.len() {
            let cell = &self.data[self.cell_pointers[i] as usize..];
            let cell = IndexInteriorCell::parse(cell)?;
            parsed_cells.push(cell);
        }

        let low =
            parsed_cells.partition_point(|x| x.get_first_column_value().as_str() < where_value);
        let high =
            parsed_cells.partition_point(|x| x.get_first_column_value().as_str() <= where_value);

        let mut pages = vec![];
        for i in low..=high {
            match i == parsed_cells.len() {
                false => {
                    pages.push(parsed_cells[i].page_number_of_left_child);
                }
                true => pages.push(self.right_most_pointer.unwrap() as usize),
            }
        }

        Ok(pages)
    }
}

#[test]
fn test_range() {
    let v = [1, 2, 3, 4, 5, 5, 5, 6, 7];
    let lo = v.partition_point(|x| x < &5);
    assert_eq!(lo, 4);
    let hi = v.partition_point(|x| x <= &5);
    assert_eq!(hi, 7);
}
