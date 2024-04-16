use super::{reader_utils::ReadeInto, serial_value::SerialValue};
use anyhow::{bail, Context, Ok, Result};
use std::io::prelude::*;
use std::io::Cursor;

#[derive(Debug)]
enum ObjectType {
    Table,
    Index,
    View,
    Trigger,
}

impl ObjectType {
    fn from(text: &str) -> Result<Self> {
        match text {
            "table" => Ok(Self::Table),
            "index" => Ok(Self::Index),
            "view" => Ok(Self::View),
            "trigger" => Ok(Self::Trigger),
            _ => bail!("Invalid object type: {}", text),
        }
    }
}

/// ref: https://www.sqlite.org/schematab.html
#[derive(Debug)]
pub struct SchemaTable {
    object_type: ObjectType,
    name: String,
    tbl_name: String,
    rootpage: Option<usize>,
    sql: String,
}

impl SchemaTable {
    pub fn from(cell: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(cell);
        let _payload_size = reader.read_varint().context("Read varint - payload size")?;
        let _row_id = reader.read_varint().context("Read varint - rowid")?;

        let header_start = reader.stream_position()?;
        let header_size = reader.read_varint().context("Read varint - header size")?;

        let mut serial_types = vec![];
        while reader.stream_position()? < header_start + header_size as u64 {
            let serial_type = reader.read_varint().context("Read varint - serial type")?;
            serial_types.push(serial_type);
        }
        assert_eq!(
            serial_types.len(),
            5,
            "SchemaTable should have exactly 5 columns"
        );

        let object_type = match reader
            .read_serial_value(serial_types[0])
            .context("Read serial value - object type")?
        {
            SerialValue::Text(text) => ObjectType::from(&text),
            _ => bail!("Object type should be text"),
        }?;

        let name = match reader
            .read_serial_value(serial_types[1])
            .context("Read serial value - name")?
        {
            SerialValue::Text(text) => text,
            _ => bail!("Name should be text"),
        };

        let tbl_name = match reader
            .read_serial_value(serial_types[2])
            .context("Read serial value - tbl name")?
        {
            SerialValue::Text(text) => text,
            _ => bail!("tbl name should be text"),
        };

        let rootpage = match reader
            .read_serial_value(serial_types[3])
            .context("Read serial value - rootpage")?
        {
            SerialValue::Int8(n) => Some(n as usize),
            SerialValue::Int16(n) => Some(n as usize),
            SerialValue::Int24(n) => Some(n as usize),
            SerialValue::Int32(n) => Some(n as usize),
            SerialValue::Int48(n) => Some(n as usize),
            SerialValue::Int64(n) => Some(n as usize),
            _ => None,
        };

        let sql = match reader
            .read_serial_value(serial_types[4])
            .context("Read serial value - sql")?
        {
            SerialValue::Text(text) => text,
            _ => bail!("sql should be text"),
        };

        Ok(Self {
            object_type,
            name,
            tbl_name,
            rootpage,
            sql,
        })
    }

    pub fn get_tbl_name(&self) -> &str {
        &self.tbl_name
    }

    pub fn get_root_page(&self) -> Option<usize> {
        self.rootpage
    }
}
