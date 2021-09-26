use crate::serialize::var::{DeserializeError, ReadVarInt, WriteVarInt};
use std::io::{Read, Write};

pub trait WriteString {
    fn write_string(&mut self, value: &str);
}

pub trait ReadString {
    fn read_string(&mut self) -> Result<String, DeserializeError>;
}

impl<T: Write> WriteString for T
{
    fn write_string(&mut self, value: &str) {
        self.write_var_int(value.len() as i32);
        self.write(value.as_bytes());
    }
}

impl<T: Read> ReadString for T
{
    fn read_string(&mut self) -> Result<String, DeserializeError> {
        match self.read_var_int() {
            Ok(len) => {
                if len <= 0 {
                    return Err(DeserializeError::InvalidLength);
                }
                let mut byte_vec = vec![0_u8; len as usize];
                self.read(byte_vec.as_mut_slice()).unwrap();
                Ok(String::from_utf8(byte_vec).unwrap())
            }
            Err(err) => Err(err),
        }
    }
}
