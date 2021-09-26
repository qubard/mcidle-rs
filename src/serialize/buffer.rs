use crate::serialize::string::{WriteString, ReadString};
use crate::serialize::var::{DeserializeError, ReadVarInt, WriteVarInt};
use std::io::{Read, Write, Error, ErrorKind};

#[derive(Clone)]
pub struct ByteBuf {
    vec: Vec<u8>,
    read_idx: usize,
}

impl From<&[u8]> for ByteBuf {
    fn from(slice: &[u8]) -> Self {
        let mut v = vec![0_u8; slice.len()];
        v.copy_from_slice(slice);
        ByteBuf {
            vec: v,
            read_idx: 0,
        }
    }
}

impl From<&Vec<u8>> for ByteBuf {
    fn from(vec: &Vec<u8>) -> Self {
        ByteBuf::from(vec.as_slice())
    }
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

    // TODO: read trait?
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

    // returns true iff buffer has `remaining` bytes available to read
    pub fn has_readable_bytes(&self, len: usize) -> bool {
        self.remaining() >= len
    }

    pub fn remaining(&self) -> usize {
        self.vec.len() - self.read_idx
    }

    // TODO: read trait?
    pub fn read_bytes(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.read_idx + len > self.len() {
            None
        } else {
            let mut dest = Vec::new();
            dest.resize(len, 0);
            dest.copy_from_slice(&self.vec.as_slice()[self.read_idx..self.read_idx + len]);
            self.read_idx += len;
            Some(dest)
        }
    }

    pub fn end(&self) -> bool {
        self.remaining() == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        self.vec.as_slice()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl Write for ByteBuf {
    fn write(&mut self, value: &[u8]) -> std::io::Result<usize> {
        // is extend_from_slice faster than this?
        self.vec.reserve(value.len());
        self.vec.write(value)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.vec.flush()
    }
}

impl Read for ByteBuf {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.read_idx + buf.len() > self.len() {
            Err(Error::new(ErrorKind::UnexpectedEof, "unexpected eof"))
        } else {
            buf.copy_from_slice(&self.vec.as_slice()[self.read_idx..self.read_idx + buf.len()]);
            self.read_idx += buf.len();
            Ok(buf.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize::buffer::*;
    use crate::serialize::string::{ReadString, WriteString};
    use crate::serialize::var::*;

    #[test]
    fn valid_varint_serialization() {
        let mut buf = ByteBuf::new();
        let mut id: i32 = 0x340;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        let mut buf = ByteBuf::new();
        id = 0x344445;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        id = -1;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        id = -2147483648;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        id = -1;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        id = 0;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());

        let mut i = 0;
        while i < 31 {
            buf = ByteBuf::new();
            id = (1 << i) - 1;
            buf.write_var_int(id);
            assert_eq!(id, buf.read_var_int().unwrap());
            i += 1;
        }
    }

    #[test]
    fn valid_string_serialization() {
        let mut buf = ByteBuf::new();
        let mut s: String = "hello".to_string();
        buf.write_string(&s);
        let x: i32 = 0xFFFEE;
        buf.write_var_int(x);
        let res = buf.read_string();
        assert!(res.is_ok());
        assert_eq!(s, res.unwrap());
        assert_eq!(x, buf.read_var_int().unwrap());
        assert_eq!(9, buf.len());

        buf = ByteBuf::new();
        s = "Привет".to_string();
        buf.write_string(&s);
        let res = buf.read_string();
        assert!(res.is_ok());
        assert_eq!(s, res.unwrap());
        assert_eq!(13, buf.len());
    }

    #[test]
    fn invalid_varint_serialization_too_small() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF_u8; 3];
        buf.write(&arr).unwrap();
        assert_eq!(arr.len(), buf.len());

        assert_eq!(
            DeserializeError::IoError,
            buf.read_var_int().unwrap_err()
        );
    }

    #[test]
    fn invalid_varint_serialization_too_big() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF_u8; 5];
        buf.write(&arr).unwrap();
        assert_eq!(arr.len(), buf.len());

        assert_eq!(
            DeserializeError::VarIntTooBig,
            buf.read_var_int().unwrap_err()
        );
    }

    #[test]
    fn varint_len_test() {
        let mut buf = ByteBuf::new();

        let x: i32 = 2147483647;
        buf.write_var_int(x);
        assert_eq!(5, buf.len());
        buf.write_var_int(0);
        assert_eq!(6, buf.len());
        buf.write_var_int(0xFF);
        assert_eq!(8, buf.len());
    }
}
