use super::{reader_utils::ReadeInto, serial_value::SerialValue};
use anyhow::{Context, Ok, Result};
use std::io::prelude::*;
use std::io::Cursor;

#[derive(Debug)]
pub struct TableLeafCell {
    pub id: u64,
    pub columns: Vec<SerialValue>,
}

impl TableLeafCell {
    pub fn parse(cell: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(cell);
        let _payload_size = reader.read_varint().context("Read varint - payload size")?;
        let row_id = reader.read_varint().context("Read varint - rowid")?;

        let header_start = reader.stream_position()?;
        let header_size = reader.read_varint().context("Read varint - header size")?;
        let mut serial_types = vec![];
        while reader.stream_position()? < header_start + header_size as u64 {
            let serial_type = reader.read_varint().context("Read varint - serial type")?;
            serial_types.push(serial_type);
        }

        let mut columns = vec![];
        for serial_type in serial_types {
            columns.push(reader.read_serial_value(serial_type)?);
        }

        Ok(Self {
            id: row_id,
            columns,
        })
    }
}
