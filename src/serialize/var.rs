use crate::ByteBuf;

#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    VarIntTooBig, // Longer than 5 bytes
    BufferTooSmall,
}

// Special trait for writing VarInt/VarLong
pub trait WriteBytesVar {
    fn write_var_int(&mut self, value: i32);
}

pub trait ReadBytesVar {
    fn read_var_int(&mut self) -> Result<i32, DeserializeError>;
}

impl WriteBytesVar for ByteBuf {
    fn write_var_int(&mut self, value: i32) {
        let mut value: i32 = value;
        if value == 0 {
            self.push(0 as u8);
        }

        while value != 0 {
            let mut current_byte: u8 = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                current_byte |= 0b10000000;
            }
            self.push(current_byte);
        }
    }
}

impl ReadBytesVar for ByteBuf {
    fn read_var_int(&mut self) -> Result<i32, DeserializeError> {
        let mut value: i32 = 0;
        let mut offset: i64 = 0;
        let mut current_byte: u8 = 0;

        while offset == 0 || (current_byte & 0b10000000) != 0 {
            if offset == 35 {
                return Err(DeserializeError::VarIntTooBig);
            }

            match self.read_byte() {
                Some(b) => {
                    current_byte = b;
                    value |= ((current_byte & 0b01111111) as i32) << offset;
                    offset += 7;
                },
                None => return Err(DeserializeError::BufferTooSmall)
            }
        }
        Ok(value)
    }
}
