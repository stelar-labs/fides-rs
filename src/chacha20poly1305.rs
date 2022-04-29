use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use rand::{RngCore, rngs::OsRng};
use std::error::Error;

pub fn encrypt(key: &[u8;32], message: &[u8]) -> Result<Vec<u8>, Box::<dyn Error>> {
    
    let key = Key::from_slice(key);
    
    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce = vec![0_u8; 12];

    OsRng.fill_bytes(&mut nonce);

    match cipher.encrypt(Nonce::from_slice(&nonce), message) {

        Ok(encrypted) => Ok([nonce, encrypted].concat()),

        Err(_) => Err("Encryption failure!")?

    }

}

pub fn decrypt(key: &[u8;32], encrypted: &[u8]) -> Result<Vec<u8>, Box::<dyn Error>> {

    let key = Key::from_slice(key);
    
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(&encrypted[..12]);

    match cipher.decrypt(nonce, encrypted[12..].as_ref()) {

        Ok(decrypted) => Ok(decrypted),
        
        Err(_) => Err("Decryption failure!")?
    
    }

}