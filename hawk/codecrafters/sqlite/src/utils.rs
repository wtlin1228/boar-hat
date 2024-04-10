use anyhow::{bail, Context, Result};
use std::io::Read;

pub fn read_bytes(reader: &mut impl Read, n_bytes: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0; n_bytes];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn read_one_byte(reader: &mut impl Read) -> Result<u8> {
    Ok(read_bytes(reader, 1)?[0])
}

/// A variable-length integer or "varint" is a static Huffman encoding of 64-bit twos-complement
/// integers that uses less space for small positive values. A varint is between 1 and 9 bytes in
/// length. The varint consists of either zero or more bytes which have the high-order bit set
/// followed by a single byte with the high-order bit clear, or nine bytes, whichever is shorter.
/// The lower seven bits of each of the first eight bytes and all 8 bits of the ninth byte are used
/// to reconstruct the 64-bit twos-complement integer. Varints are big-endian: bits taken from the
/// earlier byte of the varint are more significant than bits taken from the later bytes.
pub fn read_one_varint(reader: &mut impl Read) -> Result<u64> {
    let mut res: u64 = 0;
    // only take the lower 7 bits of each of the first eight bytes
    for _ in 1..=8 {
        let byte = read_one_byte(reader)?;
        res <<= 7;
        res += (byte & 0b0111_1111) as u64;
        if byte & 0b1000_0000 == 0 {
            return Ok(res);
        }
    }
    // take all 8 bits of the ninth byte
    let byte = read_one_byte(reader)?;
    res <<= 8;
    res += byte as u64;
    Ok(res)
}

/// https://www.sqlite.org/fileformat2.html#record_format
#[derive(Debug)]
pub enum SerialValue {
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

impl SerialValue {
    pub fn from(reader: &mut impl Read, serial_type: u64) -> Result<Self> {
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
