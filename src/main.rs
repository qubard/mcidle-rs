mod mcidle {
    use byteorder::{LittleEndian, WriteBytesExt};

    pub type ByteBuf = Vec<u8>;

    pub trait PacketSerializer {
        fn serialize(&self, buf: &mut ByteBuf);
        fn deserialize(&mut self, buf: &ByteBuf);
    }

    pub trait VarIntSerializer {
        fn write_var_int(&mut self, value: i64);
        fn read_var_int(&self, idx: usize) -> Result<i64, VarIntDecodingError>;
    }

    // We need a ToBytes trait to convert
    // arbitrary types to a bytes array since rust
    // doesn't have inheritance essentially
    //
    impl PacketSerializer for Handshake {
        fn serialize(&self, buf: &mut ByteBuf) {
            buf.write_var_int(self.protocol_version);
            write_string(buf, &self.address);
        }

        fn deserialize(&mut self, buf: &ByteBuf) {
            // these types need a cursor.... D:
            self.protocol_version = buf.read_var_int(0).unwrap();
        }
    }

    impl VarIntSerializer for ByteBuf {
        fn write_var_int(&mut self, value: i64) {
            let mut value: i64 = value;
            if value == 0 {
                self.push(0 as u8);
            }

            while value != 0 {
                let mut current_byte:u8 = (value & 0b01111111) as u8;
                value >>= 7;
                if value != 0 {
                    current_byte |= 0b10000000;
                }
                self.push(current_byte);
            }
        }

        fn read_var_int(&self, idx: usize) -> Result<i64, VarIntDecodingError> {
            let mut value: i64 = 0;
            let mut offset: i64 = 0;
            let mut current_byte:u8 = 0;
            let mut idx:usize = idx;

            while offset == 0 || (current_byte & 0b10000000) != 0 {
                if offset == 35 { 
                    return Err(VarIntDecodingError::TooBig);
                }

                unsafe {
                    if idx >= self.len() {
                        return Err(VarIntDecodingError::BufferTooSmall);
                    }

                    current_byte = *self.as_ptr().add(idx);
                    value |= ((current_byte & 0b01111111) as i64) << offset;
                    idx += 1;
                }
                offset += 7;
            }
            Ok(value)
        }
    }

    type VarInt = i64;

    #[derive(Debug)]
    enum LoginState {
        Status,
        Login
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: VarInt,
        pub address: String,
        /*
        Port: u16,
        NextState: LoginState,*/
    }

    // We have two options: using a cursor or writing our own implementation
    // that wraps a read and write cursor
    // that being said I don't think we really need anything but a read cursor
    // but having a write cursor is nice if you ever want to write onto an already
    // read buffer

    #[derive(Debug, PartialEq)]
    pub enum VarIntDecodingError {
        TooBig,
        BufferTooSmall,
    }

    pub fn write_string(buf: &mut ByteBuf, value: &String) {
        buf.write_var_int(buf.len() as i64);
        buf.write_u16::<LittleEndian>(0xFFFF).unwrap();
        buf.extend_from_slice(value.as_bytes());
    }

}

use mcidle::{ByteBuf,Handshake,PacketSerializer};

fn main() {
    // Now we want to implement it so we can write to a buffer with macro rules
    // on the ToBytes trait
    let h = Handshake{ 
        protocol_version: 340, 
        address: "localhost".to_string(), 
        /*
        Port: 25565, 
        NextState: LoginState::Login*/
    };
    let mut buf = ByteBuf::with_capacity(30);
    println!("{:#?}", h);
    h.serialize(&mut buf);
    let mut h2 = mcidle::Handshake { protocol_version: 0, address: "".to_string() };
    h2.deserialize(&buf);
    println!("{:02X?}", buf);
    println!("{}", h2.protocol_version);
}

#[cfg(test)]
mod tests {
    use crate::mcidle::*;

    #[test]
    fn valid_varint_serialization() {
        let mut buf = ByteBuf::new();
        let id: i64 = 0xFFFFFFFF;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int(0).unwrap());
    }

    #[test]
    fn invalid_varint_serialization() {
        let mut buf = ByteBuf::new();
        // overflows since we can only encode i32s in up 34 bits
        let id: i64 = 0xFFFFFFFFF;
        buf.write_var_int(id);
        assert_eq!(VarIntDecodingError::TooBig, buf.read_var_int(0).unwrap_err());
    }
}
