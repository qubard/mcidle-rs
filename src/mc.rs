use crate::serialize::buffer::ByteBuf;
use openssl::symm::{Cipher, Crypter, Mode};

use std::net::TcpStream;

use crate::encrypt::encrypt_plaintext;
use std::cell::RefCell;
use std::io::{Write, Read};
use crate::serialize::var::VarIntWriter;

type RefCrypter = RefCell<Option<Crypter>>;

pub struct Connection {
    enc: RefCrypter,
    dec: RefCrypter,
    stream: TcpStream,
}

// should we wrap our connection with a tcpstream? not sure
// it's somewhat ugly to have to initialize the cryptor later
// but it also works and is technically valid
// we don't need encryption enabled all the time so it's optional
// for a reason

impl Connection {
    pub fn new(addr: String) -> Connection {
        Connection {
            enc: RefCrypter::new(None),
            dec: RefCrypter::new(None),
            stream: TcpStream::connect(addr).unwrap(),
        }
    }

    pub fn init_cryptor(&mut self, iv: &[u8]) {
        // Note that the iv is the same as the key in Minecraft
        self.enc = RefCell::new(Some(Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, iv, Some(iv)).unwrap()));
        self.dec = RefCell::new(Some(Crypter::new(Cipher::aes_128_cbc(), Mode::Decrypt, iv, Some(iv)).unwrap()));
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        self.stream.read(buf).unwrap()
    }

    pub fn send(&mut self, buf: &ByteBuf) -> usize {
        // Prepend buffer with its length
        let mut final_buf = ByteBuf::new();
        final_buf.write_var_int(buf.len() as i32);
        final_buf.write(buf.as_slice()).unwrap();

        match self.enc.get_mut() {
            // Encrypt the buffer, then send it
            Some(cryptor) => {
                let encrypted = encrypt_plaintext(cryptor, final_buf.as_slice());
                self.stream.write(&encrypted.as_slice()).unwrap()
            }
            None => {
                self.stream.write(&final_buf.as_slice()).unwrap()
            }
        }
    }
}
