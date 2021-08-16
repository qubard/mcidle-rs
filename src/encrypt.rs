use openssl::symm::{Cipher, Crypter, Mode};

pub trait Cryptor {
    fn encrypt_plaintext(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>;
    fn decrypt_plaintext(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8>;
}

pub fn encrypt_plaintext(enc: &mut Crypter, plaintext: &[u8]) -> Vec<u8> {
    let data_len = plaintext.len();

    // Create a cipher context for encryption.
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut ciphertext = vec![0; data_len + block_size];

    let mut count = enc.update(plaintext, &mut ciphertext).unwrap();
    enc.finalize(&mut ciphertext[count..]).unwrap();
    ciphertext
}
