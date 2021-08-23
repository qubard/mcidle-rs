#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    VarIntTooBig, // Longer than 5 bytes
    BufferTooSmall,
    InvalidLength, // Length <= 0
}

// Special trait for writing VarInt/VarLong
pub trait VarIntWriter {
    fn write_var_int(&mut self, value: i32);
}

pub trait VarIntReader {
    fn read_var_int(&mut self) -> Result<(i32, i32), DeserializeError>;
}

