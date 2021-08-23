use crate::serialize::var::{VarIntWriter, VarIntReader, DeserializeError};
use crate::serialize::buffer::ByteBuf;

pub trait VarIntString: VarIntWriter + VarIntReader {
    fn len(&self) -> usize;
    fn extend_from_slice(&mut self, other: &[u8]);
}

pub trait WriteString: VarIntString {
    fn write_string(&mut self, value: &str);
}

pub trait ReadString: VarIntString {
    fn read_string(&mut self) -> Result<String, DeserializeError>;
}

impl<T> WriteString for T where T:VarIntString {
    fn write_string(&mut self, value: &str) {
        self.write_var_int(value.len() as i32);
        self.extend_from_slice(value.as_bytes());
    }
}

impl ReadString for ByteBuf {
    fn read_string(&mut self) -> Result<String, DeserializeError> {
        match self.read_var_int() {
            Ok(len) => {
                if len <= 0 {
                    return Err(DeserializeError::InvalidLength)
                }
                let byte_vec = self.read_bytes(len as usize).unwrap();
                Ok(String::from_utf8(byte_vec).unwrap())
            },
            Err(err) => Err(err)
        }
    }
}
