mod serialize;
use serialize::{ByteBuf};
use serialize::packet::*;
use serialize::string::*;
use serialize::var::*;

use crate::serialize::packet::serverbound::*;

fn main() {
    let h = Handshake {
        protocol_version: 340,
        address: "localhost".to_string(),
        port: 25565,
        next_state: LoginState::Login
    };
    let mut buf = ByteBuf::new();
    h.serialize(&mut buf);
}

#[cfg(test)]
mod tests {
    use crate::serialize::var::*;
    use crate::serialize::ByteBuf;
    use crate::serialize::bytes::*;

    #[test]
    fn valid_varint_serialization() {
        let mut buf = ByteBuf::new();
        let id: i32 = 0x340;
        buf.write_var_int(id);
        assert_eq!(id, buf.read_var_int().unwrap());
    }

    #[test]
    fn invalid_varint_serialization_too_small() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF as u8; 3];
        buf.write_bytes(&arr);
        assert_eq!(arr.len(), buf.len());

        assert_eq!(DeserializeError::BufferTooSmall, buf.read_var_int().unwrap_err());
    }

    #[test]
    fn invalid_varint_serialization_too_big() {
        let mut buf = ByteBuf::new();

        let arr = [0xFF as u8; 5];
        buf.write_bytes(&arr);
        assert_eq!(arr.len(), buf.len());

        assert_eq!(DeserializeError::VarIntTooBig, buf.read_var_int().unwrap_err());
    }

    #[test]
    fn varint_len_test() {
        let mut buf = ByteBuf::new();

        let x : i32 = 2147483647;
        buf.write_var_int(x);
        assert_eq!(5, buf.len());
        buf.write_var_int(0);
        assert_eq!(6, buf.len());
        buf.write_var_int(0xFF);
        assert_eq!(8, buf.len());
    }
}
