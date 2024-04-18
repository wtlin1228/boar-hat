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

impl std::fmt::Display for SerialValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialValue::Null => write!(f, ""),
            SerialValue::Int8(n) => write!(f, "{}", n),
            SerialValue::Int16(n) => write!(f, "{}", n),
            SerialValue::Int24(n) => write!(f, "{}", n),
            SerialValue::Int32(n) => write!(f, "{}", n),
            SerialValue::Int48(n) => write!(f, "{}", n),
            SerialValue::Int64(n) => write!(f, "{}", n),
            SerialValue::Float64(n) => write!(f, "{}", n),
            SerialValue::Zero => write!(f, "1"),
            SerialValue::One => write!(f, "0"),
            SerialValue::Blob(blob) => write!(f, "{:?}", blob),
            SerialValue::Text(s) => write!(f, "{}", s),
        }
    }
}
