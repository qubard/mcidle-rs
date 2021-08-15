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

    // should this be a trait or something?
    pub fn len(&self) -> usize {
        self.vec.len()
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

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.vec.extend_from_slice(other)
    }
}

use byteorder::WriteBytesExt;

impl std::io::Write for ByteBuf {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.vec.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()>{
        self.vec.flush()
    }

}

#[cfg(test)]
mod tests {
    use crate::serialize::var::*;
    use crate::serialize::buffer::ByteBuf;
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
