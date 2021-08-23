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
    fn read_var_int(&mut self) -> Result<i32, DeserializeError>;
}

pub fn len_varint(v: i32) -> i32 {
    let mut len = 0;
    let mut value = v;

    if v == 0 {
        return 1;
    }

    while value != 0 {
        value = ((value as u32) >> 7) as i32;
        len += 1;
    }
    len
}
