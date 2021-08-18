use crate::serialize::buffer::*;
use openssl::symm::Crypter;

use std::net::TcpStream;

use crate::encrypt::*;

use crate::serialize::packet::PacketSerializerWithID;
use crate::serialize::protocol::ProtocolVersion;
use crate::serialize::var::*;
use std::cell::RefCell;
use std::io::{Read, Write};

type RefCrypter = RefCell<Option<Crypter>>;

pub struct Connection {
    enc: RefCrypter,
    dec: RefCrypter,
    stream: TcpStream,
    ver: ProtocolVersion,
}

impl Connection {
    pub fn new(addr: String, ver: ProtocolVersion) -> Connection {
        Connection {
            enc: RefCrypter::new(None),
            dec: RefCrypter::new(None),
            stream: TcpStream::connect(addr).unwrap(),
            ver,
        }
    }

    /*pub fn init_cryptor(&mut self, iv: &[u8]) {
        // Note that the iv is the same as the key in Minecraft
        self.enc = RefCell::new(Some(Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, iv, Some(iv)).unwrap()));
        self.dec = RefCell::new(Some(Crypter::new(Cipher::aes_128_cbc(), Mode::Decrypt, iv, Some(iv)).unwrap()));
    }*/

    /*pub fn read(&mut self, buf: &mut [u8]) -> usize {
        self.stream.read(buf).unwrap()
    }*/

    pub fn send_packet(&mut self, packet: &impl PacketSerializerWithID) -> usize {
        // Write and prepend packet buffer with its length
        let buf = packet.serialize_with_id(&self.ver);
        let mut final_buf = ByteBuf::new();
        final_buf.write_var_int(buf.len() as i32);
        final_buf.write(buf.as_slice()).unwrap();

        self.send_buffer(&final_buf)
    }

    pub fn send_buffer(&mut self, buf: &ByteBuf) -> usize {
        match self.enc.get_mut() {
            // Encrypt the buffer, then send it
            Some(cryptor) => {
                let encrypted = encrypt_plaintext(cryptor, buf.as_slice());
                self.stream.write(&encrypted.as_slice()).unwrap()
            }
            None => self.stream.write(&buf.as_slice()).unwrap(),
        }
    }

    pub fn recv(&mut self) {
        let mut slice: &mut [u8] = &mut [0 as u8; 4096];
        match self.stream.read(&mut slice) {
            Ok(n) => {
                let mut buf = ByteBuf::from(&slice[..n]);
                let len = buf.read_var_int().unwrap();
                let rest = buf.read_bytes(len as usize);
                match rest {
                    Some(v) => {
                        let mut rest_buf = ByteBuf::from(v.as_slice());
                        let packet_id = (rest_buf.read_var_int().unwrap()) as u8;
                        println!("Got packet id: 0x{}", format!("{:x}", packet_id));
                    },
                    None => { 
                        panic!("unexpected none rest");
                    }
                }

                if !buf.end() {
                    panic!("NOT AT END");
                }

                println!("size: {}, len: {}, data: {}", n, len, hex::encode(&slice[..n]));
            }
            Err(e) => panic!(e),
        }
    }
}
