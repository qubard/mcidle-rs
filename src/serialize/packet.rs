use crate::serialize::buffer::ByteBuf;

pub trait PacketSerializer {
    fn serialize(&self, buf: &mut ByteBuf);
    fn deserialize(&mut self, buf: &mut ByteBuf);
}

pub mod clientbound {

}

pub mod serverbound {
    use super::PacketSerializer;
    use crate::serialize::buffer::*;

    use crate::serialize::var::*;
    use crate::serialize::string::*;

    use byteorder::{BigEndian, WriteBytesExt};

    #[derive(Debug, Clone)]
    pub enum LoginState {
        Status = 1,
        Login = 2,
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub address: String,
        pub port: u16,
        pub next_state: LoginState,
    }

    impl PacketSerializer for Handshake {
        fn serialize(&self, buf: &mut ByteBuf) {
            buf.write_var_int(self.protocol_version);
            buf.write_string(&self.address);
            buf.write_u16::<BigEndian>(self.port).unwrap();
            buf.write_var_int(self.next_state.clone() as i32);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.protocol_version = buf.read_var_int().unwrap();
        }
    }

}
