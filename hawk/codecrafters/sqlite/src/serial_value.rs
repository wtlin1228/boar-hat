/// https://www.sqlite.org/fileformat2.html#record_format
#[derive(Debug)]
pub enum SerialValue {
    Null,
    Int8(u8),
    Int16(u16),
    Int24(u32),
    Int32(u32),
    Int48(u64),
    Int64(u64),
    Float64(f64),
    Zero,
    One,
    Blob(Vec<u8>),
    Text(String),
}
