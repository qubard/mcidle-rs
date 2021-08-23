mod serialize;
use crate::serialize::packet;
use crate::serialize::packet::serverbound::*;
use crate::serialize::protocol::ProtocolVersion;

mod encrypt;
mod mc;

fn main() {
    let handshake = Handshake {
        protocol_version: 340,
        address: "localhost".to_string(),
        port: 25565,
        next_state: LoginState::Login,
    };
    let protocol = ProtocolVersion::V_1_12_2;
    let mut c = mc::Connection::new(
        "localhost:25565".to_string(),
        protocol,
        mc::BufferSize::Medium,
    );
    println!("Sent {} bytes!", c.send_packet(&handshake));

    let login_start = LoginStart {
        username: "test".to_string(),
    };
    println!("Sent {} bytes!", c.send_packet(&login_start));

    loop {
        let mut pkts = c.read_packets();
        let len = pkts.len();
        if len == 0 {
            break;
        }
        println!("Read {} packets!", len);
        for (id, buf) in pkts.iter_mut() {
            match packet::to_packet_id(*id) {
                packet::PacketID::SetCompression => {
                    let set_compression =
                        packet::deserialize_new::<packet::clientbound::SetCompression>(buf);
                    c.set_compression_threshold(set_compression.threshold);
                    println!("Compression threshold is {}!", set_compression.threshold);
                }
                packet::PacketID::KeepAliveCB => {
                    let keep_alive = packet::deserialize_new::<packet::clientbound::KeepAlive>(buf);
                    println!("Got keep alive id {}!", keep_alive.id);
                    let keep_alive_sb = packet::serverbound::KeepAlive { id: keep_alive.id };
                    c.send_packet(&keep_alive_sb);
                }
                _ => {
                    println!("Unknown packet id {:x}", id);
                }
            }
        }
        println!("done read call");
    }
}
