use crate::serialize::buffer::ByteBuf;
use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
use crate::serialize::var::VarIntWriter;

pub mod keep_alive;

pub trait PacketSerializer: ProtocolToID {
    fn serialize(&self, buf: &mut ByteBuf, ver: &ProtocolVersion);
    fn deserialize(&mut self, buf: &mut ByteBuf);
}

pub trait Packet: PacketSerializer + ProtocolToID {
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf>;
}

impl<T: PacketSerializer + ProtocolToID> Packet for T {
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf> {
        let mut buf = Box::new(ByteBuf::new());
        buf.write_var_int(self.resolve_id(ver));
        self.serialize(&mut buf, ver);
        buf
    }
}

pub fn deserialize_new<T: Default + Packet>(buf: &mut ByteBuf) -> Box<T> {
    let mut p: T = T::default();
    p.deserialize(buf);
    Box::new(p)
}

#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum PacketID {
    KeepAliveCB = 0x1F,
    KeepAliveSB = 0x0B,
    SetCompression = 0x03,
    Handshake = 0x00,
    EncryptionRequest = 0x01,
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Serializer {
    buf: ByteBuf,
}

pub fn to_packet_id(id: i32) -> PacketID {
    let packet_id: PacketID = unsafe { ::std::mem::transmute(id) };
    packet_id
}

pub mod clientbound {
    use super::{PacketID, PacketSerializer};
    use crate::serialize::buffer::ByteBuf;
    use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
    use crate::serialize::var::*;

    #[derive(Debug, Default)]
    pub struct SetCompression {
        pub threshold: i32,
    }

    impl ProtocolToID for SetCompression {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            PacketID::SetCompression as i32
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
    use super::{PacketID, PacketSerializer};

    use crate::serialize::buffer::*;
    use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
    use crate::serialize::string::*;
    use crate::serialize::var::*;

    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    #[derive(Debug, Clone, PartialEq)]
    #[repr(i32)]
    pub enum LoginState {
        Undefined = 0,
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

    #[derive(Debug, Default)]
    pub struct LoginStart {
        pub username: String,
    }

    impl ProtocolToID for Handshake {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            PacketID::Handshake as i32
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
            self.next_state = unsafe { ::std::mem::transmute(buf.read_var_int().unwrap()) };
        }
    }

    impl ProtocolToID for LoginStart {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            0x00
        }
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
    use crate::serialize::var::VarIntReader;

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
        let h2 = deserialize_new::<Handshake>(&mut buf);
        assert_eq!(h.protocol_version, h2.protocol_version);
        assert_eq!(h.address, h2.address);
        assert_eq!(h.port, h2.port);
        assert_eq!(h.next_state, h2.next_state);
    }
}
