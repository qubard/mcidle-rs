mod serialize;
use serialize::buffer::ByteBuf;
use serialize::packet::*;
use crate::serialize::packet::serverbound::*;

mod encrypt;
use encrypt::*;
use hex;

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
    println!("Sent {} bytes!", c.send(&buf));

    let l = LoginStart {
        username: "test".to_string(),
    };
    let mut buf = ByteBuf::new();
    l.serialize(&mut buf);
    println!("Sent {} bytes!", c.send(&buf));

    let mut arr = [0 as u8; 4096];
    while true {
        let nread = c.read(&mut arr);
        println!("Read {} bytes!", nread);
        let b: &[u8] = &arr[0..nread];
        println!("{}", hex::encode(b));
        if nread == 0 {
            break
        }
    }
}

