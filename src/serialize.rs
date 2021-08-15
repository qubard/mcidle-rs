pub mod bytes;
pub mod packet;
pub mod string;
pub mod var;

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
