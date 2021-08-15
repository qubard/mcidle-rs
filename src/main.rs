mod serialize;
use serialize::buffer::ByteBuf;
use serialize::packet::*;
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

