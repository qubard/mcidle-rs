use crate::serialize::packet::{PacketID, PacketSerializer};
use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::serialize::buffer::ByteBuf;

pub mod clientbound {
    use super::*;

    #[derive(Debug, Default)]
    pub struct KeepAlive {
        pub id: i64,
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            PacketID::KeepAliveCB as i32
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
    use super::*;

    #[derive(Debug, Default)]
    pub struct KeepAlive {
        pub id: i64,
    }

    impl ProtocolToID for KeepAlive {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            PacketID::KeepAliveSB as i32
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
