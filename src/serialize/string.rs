use byteorder::{LittleEndian, WriteBytesExt};
use crate::serialize::ByteBuf;
use crate::serialize::var::*;

pub trait WriteString {
    fn write_string(&mut self, value: &String);
}

impl WriteString for ByteBuf {
    fn write_string(&mut self, value: &String) {
        self.write_var_int(self.len() as i32);
        self.extend_from_slice(value.as_bytes());
    }
}

