use super::{
    reader_utils::ReadeInto,
    serial_value::SerialValue,
    sql_parser::{CreateIndexStmt, CreateTableStmt, SQLParser},
};
use anyhow::{bail, Context, Ok, Result};
use std::io::prelude::*;
use std::io::Cursor;

#[derive(Debug)]
pub enum ObjectType {
    Table(CreateTableStmt),
    Index(CreateIndexStmt),
    View,
    Trigger,
}

impl ObjectType {
    pub fn is_table(&self) -> bool {
        match self {
            ObjectType::Table(_) => true,
            _ => false,
        }
    }
}

/// ref: https://www.sqlite.org/schematab.html
#[derive(Debug)]
pub struct SchemaTable {
    pub object_type: ObjectType,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: Option<usize>,
    pub sql: String,
}

impl SchemaTable {
    // Can also use `super::cell::TableLeafCell::parse` then fill each column into SchemaTable's fields.
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
            SerialValue::Text(text) => text,
            _ => bail!("Object type should be text"),
        };

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

        let object_type = match object_type.as_str() {
            "table" => ObjectType::Table(SQLParser::parse_create_table_stmt(&sql).context(
                format!("Fail to parse create table's sql statement: {}", sql),
            )?),
            "index" => ObjectType::Index(SQLParser::parse_create_index_stmt(&sql).context(
                format!("Fail to parse create index's sql statement: {}", sql),
            )?),
            "view" => ObjectType::View,
            "trigger" => ObjectType::Trigger,
            _ => bail!("Invalid object type: {}", object_type),
        };

        Ok(Self {
            object_type,
            name,
            tbl_name,
            rootpage,
            sql,
        })
    }

    pub fn get_table_column_def(&self) -> Result<&[String]> {
        match self.object_type {
            ObjectType::Table(ref stmt) => Ok(&stmt.column_def),
            _ => bail!(
                "Column definition doesn't be declared on {:?} object type",
                self.object_type
            ),
        }
    }

    // pub fn get_index(table_name: &str, column_name: &str) -> Option<&SchemaTable> {

    // }
}
