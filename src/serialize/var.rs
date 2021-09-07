use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
pub enum DeserializeError {
    VarIntTooBig, // Longer than 5 bytes
    InvalidLength, // Length <= 0
    IoError,
}

// Special trait for writing VarInt/VarLong
pub trait WriteVarInt {
    fn write_var_int(&mut self, value: i32);
}

pub trait ReadVarInt {
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

impl<T> WriteVarInt for T
where
    T: Write,
{
    fn write_var_int(&mut self, value: i32) {
        let mut value: i32 = value;
        if value == 0 {
            //self.push(0_u8);
            self.write_u8(0_u8).unwrap();
        }

        while value != 0 {
            let mut current_byte: u8 = (value & 0b01111111) as u8;
            // unsigned right shift
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                current_byte |= 0b10000000;
            }
            self.write_u8(current_byte).unwrap();
        }
    }
}

impl<T> ReadVarInt for T where T: Read {
    fn read_var_int(&mut self) -> Result<i32, DeserializeError> {
        let mut value: i32 = 0;
        let mut offset: i64 = 0;
        let mut current_byte: u8 = 0;

        while offset == 0 || (current_byte & 0b10000000) != 0 {
            if offset == 35 {
                return Err(DeserializeError::VarIntTooBig);
            }

            match self.read_u8() {
                Ok(b) => {
                    current_byte = b;
                    value |= ((current_byte & 0b01111111) as i32) << offset;
                    offset += 7;
                }
                Err(e) => return Err(DeserializeError::IoError),
            }
        }
        Ok(value)
    }
}
