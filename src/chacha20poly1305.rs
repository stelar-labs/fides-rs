use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use rand::{RngCore, rngs::OsRng};
use std::error::Error;

pub fn encrypt(key: &[u8;32], plain_msg: &Vec<u8>) -> Result<Vec<u8>, Box::<dyn Error>> {
    
    let key = Key::from_slice(key);
    
    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce = vec![0_u8; 12];

    OsRng.fill_bytes(&mut nonce);

    match cipher.encrypt(Nonce::from_slice(&nonce), plain_msg.as_ref()) {
        Ok(r) => Ok([nonce, r].concat()),
        Err(_) => Err("Encryption failure!")?
    }

}

pub fn decrypt(key: &[u8;32], cipher_msg: &Vec<u8>) -> Result<Vec<u8>, Box::<dyn Error>> {

    let key = Key::from_slice(key);
    
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(&cipher_msg[..12]);

    match cipher.decrypt(nonce, cipher_msg[12..].as_ref()) {
        Ok(r) => Ok(r),
        Err(_) => Err("Decryption failure!")?
    }

}