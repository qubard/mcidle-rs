use crate::serialize::packet::{PacketID, PacketSerializer};
use crate::serialize::protocol::{ProtocolToID, ProtocolVersion};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::serialize::buffer::ByteBuf;

pub mod clientbound {
    use super::*;

    #[derive(Debug, Default)]
    pub struct EncryptionRequest {
    }

    impl ProtocolToID for EncryptionRequest {
        fn resolve_id(&self, _ver: &ProtocolVersion) -> i32 {
            PacketID::EncryptionRequest as i32
        }
    }

    impl PacketSerializer for EncryptionRequest {
        fn serialize(&self, buf: &mut ByteBuf, _: &ProtocolVersion) {
            buf.write_i64::<BigEndian>(self.id).unwrap();
        }

        fn deserialize(&mut self, buf: &mut ByteBuf) {
            self.id = buf.read_i64::<BigEndian>().unwrap();
        }
    }
}
