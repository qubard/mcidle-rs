use crate::serialize::buffer::ByteBuf;

pub trait PacketSerializer {
    fn serialize(&self, buf: &mut ByteBuf);
    fn deserialize(&mut self, buf: &mut ByteBuf);
}

pub mod clientbound {}

pub mod serverbound {
    use super::PacketSerializer;
    use crate::serialize::buffer::*;

    use crate::serialize::string::*;
    use crate::serialize::var::*;

    use byteorder::{BigEndian, WriteBytesExt};

    #[derive(Debug, Clone)]
    pub enum LoginState {
        Undefined = 0,
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

    #[derive(Debug)]
    pub struct LoginStart {
        pub username: String,
    }

    impl PacketSerializer for Handshake {
        fn serialize(&self, buf: &mut ByteBuf) {
            buf.write_var_int(0x00); // packet id
            buf.write_var_int(self.protocol_version);
            buf.write_string(&self.address);
            buf.write_u16::<BigEndian>(self.port).unwrap();
            buf.write_var_int(self.next_state.clone() as i32);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.protocol_version = buf.read_var_int().unwrap();
        }
    }

    impl PacketSerializer for LoginStart {
        fn serialize(&self, buf: &mut ByteBuf) {
            buf.write_var_int(0x00); // packet id
            buf.write_string(&self.username);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            //self.username = buf.read_string().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize::buffer::*;
    use crate::serialize::packet::serverbound::*;
    use crate::serialize::packet::*;

    #[test]
    fn valid_serialize() {
        let h = Handshake {
            protocol_version: 340,
            address: "localhost".to_string(),
            port: 25565,
            next_state: LoginState::Login,
        };
        let mut buf = ByteBuf::new();
        h.serialize(&mut buf);
        let mut h2 = Handshake {
            protocol_version: 0,
            address: "".to_string(),
            port: 0,
            next_state: LoginState::Undefined,
        };
        h2.deserialize(&mut buf);
    }
}
