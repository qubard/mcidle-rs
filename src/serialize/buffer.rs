use crate::serialize::var::{VarIntReader, VarIntWriter, DeserializeError};
use crate::serialize::string::StringWriter;

pub struct ByteBuf {
    vec: Vec<u8>,
    pub read_idx: usize,
}

impl ByteBuf {
    pub fn new() -> ByteBuf {
        ByteBuf {
            vec: Vec::new(),
            read_idx: 0,
        }
    }

    pub fn push(&mut self, v: u8) {
        self.vec.push(v)
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        unsafe {
            if self.read_idx >= self.len() {
                None
            } else {
                let read_idx_byte = *self.vec.as_ptr().add(self.read_idx);
                self.read_idx += 1;
                Some(read_idx_byte)
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.vec.as_slice()
    }
}


impl StringWriter for ByteBuf {
    fn len(&self) -> usize {
        self.vec.len()
    }

    fn extend_from_slice(&mut self, other: &[u8]) {
        self.vec.extend_from_slice(other)
    }
}

impl std::io::Write for ByteBuf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.vec.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()>{
        self.vec.flush()
    }
}

impl VarIntWriter for ByteBuf {
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

impl VarIntReader for ByteBuf {
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

#[cfg(test)]
mod tests {
    use crate::serialize::var::*;
    use crate::serialize::buffer::*;
    use crate::serialize::bytes::*;

    #[test]
    fn valid_varint_serialization() {
        let mut buf = ByteBuf::new();
        let id: i32 = 0x340;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());
    }

    #[test]
    fn invalid_varint_serialization_too_small() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF as u8; 3];
        buf.write_bytes(&arr);
        assert_eq!(arr.len(), buf.len());

        assert_eq!(DeserializeError::BufferTooSmall, buf.read_var_int().unwrap_err());
    }

    #[test]
    fn invalid_varint_serialization_too_big() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF as u8; 5];
        buf.write_bytes(&arr);
        assert_eq!(arr.len(), buf.len());

        assert_eq!(DeserializeError::VarIntTooBig, buf.read_var_int().unwrap_err());
    }

    #[test]
    fn varint_len_test() {
        let mut buf = ByteBuf::new();

        let x : i32 = 2147483647;
        buf.write_var_int(x);
        assert_eq!(5, buf.len());
        buf.write_var_int(0);
        assert_eq!(6, buf.len());
        buf.write_var_int(0xFF);
        assert_eq!(8, buf.len());
    }
}
