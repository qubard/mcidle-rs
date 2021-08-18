use crate::serialize::buffer::*;
use crate::serialize::string::VarIntString;

pub trait WriteBytes {
    fn write_bytes(&mut self, value: &[u8]);
}

impl WriteBytes for ByteBuf {
    fn write_bytes(&mut self, value: &[u8]) {
        self.extend_from_slice(value);
    }
}
