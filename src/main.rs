mod serialize;
use serialize::buffer::ByteBuf;
use serialize::packet::*;
use crate::serialize::packet::serverbound::*;

mod encrypt;
use encrypt::*;

mod mc;

fn main() {
    let h = Handshake {
        protocol_version: 340,
        address: "localhost".to_string(),
        port: 25565,
        next_state: LoginState::Login
    };
    let mut buf = ByteBuf::new();
    h.serialize(&mut buf);
    let iv = vec![0 as u8];
    let mut c = mc::Connection::new("localhost:25565".to_string());
    c.send(&buf);


    let l = LoginStart {
        username: "test".to_string(),
    };
    buf = ByteBuf::new();
    l.serialize(&mut buf);
    c.send(&buf);
}

