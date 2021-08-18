use crate::serialize::buffer::ByteBuf;
use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
use crate::serialize::var::{VarIntWriter, VarIntReader};

pub trait PacketSerializer: ProtocolToID {
    fn serialize(&self, buf: &mut ByteBuf, ver: &ProtocolVersion);
    fn deserialize(&mut self, buf: &mut ByteBuf);
}

pub trait PacketSerializerWithID: PacketSerializer + ProtocolToID {
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf>;
    fn deserialize_with_id(&mut self, buf: &mut ByteBuf);
}

impl<T> PacketSerializerWithID for T
where
    T: PacketSerializer + ProtocolToID,
{
    fn serialize_with_id(&self, ver: &ProtocolVersion) -> Box<ByteBuf> {
        let mut buf = ByteBuf::new();
        buf.write_var_int(self.resolve_id(ver));
        self.serialize(&mut buf, &ver);
        Box::new(buf)
    }

    fn deserialize_with_id(&mut self, buf: &mut ByteBuf) {
        buf.read_var_int().unwrap(); // read off packet id, do nothing for now
        self.deserialize(buf);
    }
}

pub mod clientbound {
    use super::PacketSerializer;
    use crate::serialize::protocol::{ProtocolVersion, ProtocolToID};
    use crate::serialize::buffer::ByteBuf;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    #[derive(Debug)]
    pub struct KeepAlive {
        pub id: i64,
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => 0x1F,
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
}

pub mod serverbound {
    use super::PacketSerializer;

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

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct LoginStart {
        pub username: String,
    }

    impl ProtocolToID for Handshake {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => 0x00,
            }
        }
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, ver: &ProtocolVersion) -> i32 {
            match ver {
                _ => 0x0B,
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

        let mut h2 = Handshake {
            protocol_version: 0,
            address: "".to_string(),
            port: 0,
            next_state: LoginState::Undefined,
        };

        h2.deserialize_with_id(&mut buf);
        assert_eq!(h.protocol_version, h2.protocol_version);
        assert_eq!(h.address, h2.address);
        assert_eq!(h.port, h2.port);
        assert_eq!(h.next_state, h2.next_state);
    }
}
