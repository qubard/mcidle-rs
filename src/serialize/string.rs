use crate::serialize::var::VarIntWriter;

pub trait StringWriter: VarIntWriter {
    fn len(&self) -> usize;
    fn extend_from_slice(&mut self, other: &[u8]);
}

pub trait WriteString: StringWriter {
    fn write_string(&mut self, value: &String);
}

impl<T> WriteString for T where T:StringWriter {
    fn write_string(&mut self, value: &String) {
        self.write_var_int(value.len() as i32);
        self.extend_from_slice(value.as_bytes());
    }
}

