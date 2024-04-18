use super::serial_value::SerialValue;
use anyhow::{bail, Result};
use std::io::Read;

pub trait ReadeInto {
    fn read_byte(&mut self) -> Result<u8>;
    fn read_bytes(&mut self, byte_count: usize) -> Result<Vec<u8>>;
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self, byte_count: usize) -> Result<u16>;
    fn read_u32(&mut self, byte_count: usize) -> Result<u32>;
    fn read_u64(&mut self, byte_count: usize) -> Result<u64>;
    fn read_f64(&mut self, byte_count: usize) -> Result<f64>;
    fn read_blob(&mut self, byte_count: usize) -> Result<Vec<u8>>;
    fn read_text(&mut self, byte_count: usize) -> Result<String>;
    fn read_varint(&mut self) -> Result<u64>;
    fn read_serial_value(&mut self, serial_type: u64) -> Result<SerialValue>;
}

macro_rules! extend_bytes {
    ($extend_to_size:expr, $from_bytes:expr) => {{
        assert!(
            $from_bytes.len() <= $extend_to_size,
            "Can't extend bytes with size {} to size {}",
            $from_bytes.len(),
            $extend_to_size
        );
        let mut offset = $extend_to_size - $from_bytes.len();
        let mut bytes = [0u8; $extend_to_size];
        for byte in $from_bytes.iter() {
            bytes[offset] = *byte;
            offset += 1;
        }
        bytes
    }};
}

impl<Reader: Read> ReadeInto for Reader {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_bytes(&mut self, byte_count: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; byte_count];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(u8::from_be_bytes([self.read_byte()?]))
    }

    fn read_u16(&mut self, byte_count: usize) -> Result<u16> {
        let read_bytes = self.read_bytes(byte_count)?;
        let bytes = extend_bytes![2, read_bytes];
        Ok(u16::from_be_bytes(bytes))
    }

    fn read_u32(&mut self, byte_count: usize) -> Result<u32> {
        let read_bytes = self.read_bytes(byte_count)?;
        let bytes = extend_bytes![4, read_bytes];
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_u64(&mut self, byte_count: usize) -> Result<u64> {
        let read_bytes = self.read_bytes(byte_count)?;
        let bytes = extend_bytes![8, read_bytes];
        Ok(u64::from_be_bytes(bytes))
    }

    fn read_f64(&mut self, byte_count: usize) -> Result<f64> {
        let read_bytes = self.read_bytes(byte_count)?;
        let bytes = extend_bytes![8, read_bytes];
        Ok(f64::from_be_bytes(bytes))
    }

    fn read_blob(&mut self, byte_count: usize) -> Result<Vec<u8>> {
        Ok(self.read_bytes(byte_count)?)
    }

    fn read_text(&mut self, byte_count: usize) -> Result<String> {
        Ok(String::from_utf8(self.read_bytes(byte_count)?)?)
    }

    /// A variable-length integer or "varint" is a static Huffman encoding of 64-bit twos-complement
    /// integers that uses less space for small positive values. A varint is between 1 and 9 bytes in
    /// length. The varint consists of either zero or more bytes which have the high-order bit set
    /// followed by a single byte with the high-order bit clear, or nine bytes, whichever is shorter.
    /// The lower seven bits of each of the first eight bytes and all 8 bits of the ninth byte are used
    /// to reconstruct the 64-bit twos-complement integer. Varints are big-endian: bits taken from the
    /// earlier byte of the varint are more significant than bits taken from the later bytes.
    fn read_varint(&mut self) -> Result<u64> {
        let mut res: u64 = 0;
        // only take the lower 7 bits of each of the first eight bytes
        for _ in 1..=8 {
            let byte = self.read_byte()?;
            res <<= 7;
            res += (byte & 0b0111_1111) as u64;
            if byte & 0b1000_0000 == 0 {
                return Ok(res);
            }
        }
        // take all 8 bits of the ninth byte
        let byte = self.read_byte()?;
        res <<= 8;
        res += byte as u64;
        Ok(res)
    }

    fn read_serial_value(&mut self, serial_type: u64) -> Result<SerialValue> {
        match serial_type {
            0 => Ok(SerialValue::Null),
            1 => Ok(SerialValue::Int8(self.read_u8()?)),
            2 => Ok(SerialValue::Int16(self.read_u16(2)?)),
            3 => Ok(SerialValue::Int24(self.read_u32(3)?)),
            4 => Ok(SerialValue::Int32(self.read_u32(4)?)),
            5 => Ok(SerialValue::Int48(self.read_u64(6)?)),
            6 => Ok(SerialValue::Int64(self.read_u64(8)?)),
            7 => Ok(SerialValue::Float64(self.read_f64(8)?)),
            8 => Ok(SerialValue::Zero),
            9 => Ok(SerialValue::One),
            10 | 11 => bail!("Reserved for internal use."),
            n if n >= 12 && n % 2 == 0 => {
                Ok(SerialValue::Blob(self.read_blob(((n - 12) / 2) as usize)?))
            }
            n if n >= 13 && n % 2 == 1 => {
                Ok(SerialValue::Text(self.read_text(((n - 13) / 2) as usize)?))
            }
            _ => bail!("Invalid serial type: {}", serial_type),
        }
    }
}
