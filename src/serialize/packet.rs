use crate::serialize::buffer::ByteBuf;
use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
use crate::serialize::var::{VarIntReader, VarIntWriter};

pub trait PacketSerializer: ProtocolToID {
    fn serialize(&self, buf: &mut ByteBuf, ver: &ProtocolVersion);
    fn deserialize(&mut self, buf: &mut ByteBuf);
}

pub trait Packet: PacketSerializer + ProtocolToID + Default + PacketHandler {
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf>;
    fn deserialize_gen(buf: &mut ByteBuf) -> Box<Self>;
}

impl<T: PacketSerializer + ProtocolToID + PacketHandler + Default> Packet for T
{
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf> {
        let mut buf = Box::new(ByteBuf::new());
        buf.write_var_int(self.resolve_id(ver));
        self.serialize(&mut buf, &ver);
        buf
    }

    fn deserialize_gen(buf: &mut ByteBuf) -> Box<T> {
        let mut p: T = Default::default();
        p.deserialize(buf);
        Box::new(p)
    }
}

pub trait PacketHandler {
    fn handle(&self);
}

#[derive(PartialEq)]
pub enum PacketID {
    KeepAliveCB = 0x1F,
    KeepAliveSB = 0x0B,
    SetCompression = 0x03,
    Handshake = 0x00,
}

pub fn deserialize_packet(id: i32, buf: &mut ByteBuf) -> Option<Box<dyn PacketHandler>> {
    match id {
        0x03 => { 
            Some(clientbound::SetCompression::deserialize_gen(buf))
        }
        _ => { 
            None
        }
    }
}

pub mod clientbound {
    use super::{PacketHandler, PacketID, PacketSerializer};
    use crate::serialize::buffer::ByteBuf;
    use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
    use crate::serialize::var::*;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    // TODO: custom derive PacketHandler
    #[derive(Debug, Default)]
    pub struct KeepAlive {
        pub id: i64,
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => PacketID::KeepAliveCB as i32,
            }
        }
    }

    impl PacketSerializer for KeepAlive {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_i64::<BigEndian>(self.id).unwrap();
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.id = buf.read_i64::<BigEndian>().unwrap();
        }
    }

    #[derive(Debug, Default)]
    pub struct SetCompression {
        pub threshold: i32,
    }

    impl PacketHandler for SetCompression {
        fn handle(&self) {
            println!("my threshold is : {}", self.threshold);
        }
    }

    impl ProtocolToID for SetCompression {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => PacketID::SetCompression as i32,
            }
        }
    }

    impl PacketSerializer for SetCompression {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_var_int(self.threshold);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.threshold = buf.read_var_int().unwrap();
        }
    }
}

pub mod serverbound {
    use super::{PacketHandler, PacketID, PacketSerializer};

    use crate::serialize::buffer::*;
    use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
    use crate::serialize::string::*;
    use crate::serialize::var::*;

    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoginState {
        Undefined = 0,
        Status = 1,
        Login = 2,
    }

    impl Default for LoginState {
        fn default() -> Self {
            LoginState::Undefined
        }
    }

    #[derive(Debug, Default)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub address: String,
        pub port: u16,
        pub next_state: LoginState,
    }

    #[derive(Debug)]
    pub struct KeepAlive {
        pub id: i64,
    }

    #[derive(Debug, Default)]
    pub struct LoginStart {
        pub username: String,
    }

    impl ProtocolToID for Handshake {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => PacketID::Handshake as i32,
            }
        }
    }

    impl PacketHandler for Handshake {
        fn handle(&self) {
            println!("Handled handshake");
        }
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => PacketID::KeepAliveSB as i32,
            }
        }
    }

    impl PacketSerializer for KeepAlive {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_i64::<BigEndian>(self.id).unwrap();
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.id = buf.read_i64::<BigEndian>().unwrap();
        }
    }

    impl PacketSerializer for Handshake {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_var_int(self.protocol_version);
            buf.write_string(&self.address);
            buf.write_u16::<BigEndian>(self.port).unwrap();
            buf.write_var_int(self.next_state.clone() as i32);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.protocol_version = buf.read_var_int().unwrap();
            self.address = buf.read_string().unwrap();
            self.port = buf.read_u16::<BigEndian>().unwrap();
            match buf.read_var_int().unwrap() {
                1 => {
                    self.next_state = LoginState::Status;
                }
                2 => self.next_state = LoginState::Login,
                _ => self.next_state = LoginState::Undefined,
            };
        }
    }

    impl ProtocolToID for LoginStart {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => 0x00,
            }
        }
    }

    impl PacketHandler for LoginStart {
        fn handle(&self) {}
    }

    impl PacketSerializer for LoginStart {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_string(&self.username);
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.username = buf.read_string().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PacketID;
    use crate::serialize::packet::serverbound::*;
    use crate::serialize::packet::*;

    #[test]
    fn valid_handshake_test() {
        let h = Handshake {
            protocol_version: 340,
            address: "localhost".to_string(),
            port: 25565,
            next_state: LoginState::Login,
        };

        let mut buf = h.serialize_with_id(&ProtocolVersion::V_1_12_2);
        assert_eq!(16, buf.len());

        assert_eq!(PacketID::Handshake as i32, buf.read_var_int().unwrap());
        let h2 = serverbound::Handshake::deserialize_gen(buf.as_mut());
        assert_eq!(h.protocol_version, h2.protocol_version);
        assert_eq!(h.address, h2.address);
        assert_eq!(h.port, h2.port);
        assert_eq!(h.next_state, h2.next_state);
    }
}
