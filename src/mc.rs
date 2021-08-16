use openssl::symm::{Cipher, Mode, Crypter};
use crate::serialize::buffer::ByteBuf;

use std::net::TcpStream;

use crate::encrypt::encrypt_plaintext;
use std::io::Write;
use std::cell::RefCell;

pub struct Connection {
    enc: RefCell<Option<Crypter>>,
    stream: TcpStream,
    //dec: Crypter,
}

impl Connection {
    pub fn new(addr: String) -> Connection {
        Connection { enc: RefCell::new(None), stream: TcpStream::connect(addr).unwrap() }
    }

    pub fn initialize_encryptor(&mut self, iv: &[u8]) {
        // Note that the iv is the same as the key in Minecraft
        let enc = Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, iv, Some(iv)).unwrap();
        self.enc = RefCell::new(Some(enc))
    }

    pub fn send(&mut self, buf: &ByteBuf) {
        // Encrypt the buffer, then send it
        // ???
        //
        println!("{:X?}", buf.as_slice());
        match self.enc.get_mut() {
            Some(cryptor) => {
                let out = encrypt_plaintext(cryptor, buf.as_slice());
                self.stream.write(&out).unwrap();
            },
            None => {
                self.stream.write(&buf.as_slice()).unwrap();
            }
        }
    }
}
