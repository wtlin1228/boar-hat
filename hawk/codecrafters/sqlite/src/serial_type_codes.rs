use anyhow::{bail, Context, Result};
use std::io::Read;

#[derive(Debug)]
pub enum Record {
    Null,
    Int8(i8),
    Int16(i16),
    Int24(i32),
    Int32(i32),
    Int48(i64),
    Int64(i64),
    Float64(f64),
    Zero,
    One,
    Blob(Vec<u8>),
    Text(String),
}

impl Record {
    pub fn from(reader: &mut impl Read) -> Result<Self> {
        macro_rules! read_exact {
            ($size:expr) => {{
                let mut buff = [0; $size];
                reader.read_exact(&mut buff)?;
                buff
            }};

            ($padding:expr, $size:expr) => {{
                let buff = read_exact!($size);
                let mut buff_with_padding = [0; $padding + $size];
                for i in 0..buff.len() {
                    buff_with_padding[$padding + i] = buff[i];
                }
                buff_with_padding
            }};
        }

        // TODO: need to implement the variable-length integer (varint) to read those
        // values, ref: https://www.sqlite.org/fileformat2.html#varint
        let payload_size = i64::from_be_bytes(read_exact!(8));
        let row_id = i64::from_be_bytes(read_exact!(8));
        let expected_header_size = i64::from_be_bytes(read_exact!(8));
        let serial_type = i64::from_be_bytes(read_exact!(8));
        match serial_type {
            0 => Ok(Self::Null),
            1 => Ok(Self::Int8(i8::from_be_bytes(read_exact!(1)))),
            2 => Ok(Self::Int16(i16::from_be_bytes(read_exact!(2)))),
            3 => Ok(Self::Int24(i32::from_be_bytes(read_exact!(1, 3)))),
            4 => Ok(Self::Int32(i32::from_be_bytes(read_exact!(4)))),
            5 => Ok(Self::Int48(i64::from_be_bytes(read_exact!(2, 6)))),
            6 => Ok(Self::Int64(i64::from_be_bytes(read_exact!(8)))),
            7 => Ok(Self::Float64(f64::from_be_bytes(read_exact!(8)))),
            8 => Ok(Self::Zero),
            9 => Ok(Self::One),
            10 | 11 => bail!("Reserved for internal use."),
            n if n >= 12 && n % 2 == 0 => Ok(Self::Blob({
                let content_size = (n - 12) / 2;
                let mut buff = vec![0; content_size as usize];
                reader.read_exact(&mut buff)?;
                buff
            })),
            n if n >= 13 && n % 2 == 1 => Ok(Self::Text({
                let content_size = (n - 13) / 2;
                let mut buff = vec![0; content_size as usize];
                reader.read_exact(&mut buff)?;
                String::from_utf8(buff).context("Reading a text record")?
            })),
            _ => bail!("Invalid serial type: {}", serial_type),
        }
    }
}
