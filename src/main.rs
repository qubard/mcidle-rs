mod serialize;
use crate::serialize::packet::serverbound::*;
use crate::serialize::protocol::ProtocolVersion;

mod encrypt;

use hex;

mod mc;

fn main() {
    let handshake = Handshake {
        protocol_version: 340,
        address: "localhost".to_string(),
        port: 25565,
        next_state: LoginState::Login
    };
    let protocol = ProtocolVersion::V_1_12_2;
    let mut c = mc::Connection::new("localhost:25565".to_string(), protocol);
    println!("Sent {} bytes!", c.send_packet(&handshake));

    let login_start = LoginStart {
        username: "test".to_string(),
    };
    println!("Sent {} bytes!", c.send_packet(&login_start));

    let mut arr = [0 as u8; 4096];
    loop {
        c.recv();
    }
}

