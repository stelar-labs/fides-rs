use opis::Bit;
mod bloom_filter;
pub mod hash;
pub mod merkle_tree;
pub mod ed25519;
pub mod x25519;

#[derive(Debug)]
pub struct BloomFilter { bits: Vec<Bit> }

use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use rand::{RngCore, rngs::OsRng};
use std::error::Error;

pub fn encrypt(key: &[u8;32], message: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box::<dyn Error>> {
    
    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let mut nonce = vec![0_u8; 12];
    OsRng.fill_bytes(&mut nonce);

    match cipher.encrypt(Nonce::from_slice(&nonce), message) {
        Ok(encrypted) => Ok((nonce, encrypted)),
        Err(_) => Err("Encryption failure!")?
    }

}

pub fn decrypt(key: &[u8;32], nonce: &[u8;12], encrypted: &[u8]) -> Result<Vec<u8>, Box::<dyn Error>> {

    let key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(key);
    let nonce = Nonce::from_slice(nonce);

    match cipher.decrypt(nonce, encrypted.as_ref()) {
        Ok(decrypted) => Ok(decrypted),
        Err(_) => Err("Decryption failure!")?
    }

}
