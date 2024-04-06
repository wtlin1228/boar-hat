use anyhow::{bail, Result};

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct DatabaseFileHeader {
    header_string: [u8; 16],
    pub page_size: [u8; 2],
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

impl DatabaseFileHeader {
    pub const DATABASE_FILE_HEADER_SIZE: usize = std::mem::size_of::<DatabaseFileHeader>();

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() != Self::DATABASE_FILE_HEADER_SIZE {
            bail!("Fail construct database header from input data due to length mismatch");
        }

        let d = data as *const [u8] as *const DatabaseFileHeader;
        Ok(unsafe { *d.clone() })
    }
}
