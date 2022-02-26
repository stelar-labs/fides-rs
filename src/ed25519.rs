use rand::{RngCore, rngs::OsRng};
use ed25519_dalek::{ Keypair, PublicKey, Verifier, SecretKey, Signature, Signer };

pub fn private_key() -> [u8; 32] {

    let mut private_key: [u8; 32] = [0_u8; 32];
    
    let mut key = [0_u8; 32];

    OsRng.fill_bytes(&mut key);

    while private_key == [0_u8; 32] {
    
        match SecretKey::from_bytes(&key) {
            Ok(r) => private_key = r.to_bytes(),
            Err(_) => OsRng.fill_bytes(&mut key)
        }

    }

    private_key

}

pub fn public_key(priv_key: &[u8;32]) -> [u8;32] {

    let private_key: SecretKey = SecretKey::from_bytes(&*priv_key).unwrap();

    let public_key: PublicKey = PublicKey::from(&private_key);

    public_key.to_bytes()

}

pub fn sign(message: &[u8; 32], priv_key: &[u8; 32], pub_key: &[u8; 32]) -> [u8;64] {

    let key_pair_bytes: Vec<u8> = [priv_key.to_vec(), pub_key.to_vec()].concat();

    let key_pair: Keypair = Keypair::from_bytes(&key_pair_bytes).unwrap();

    let signature: Signature = key_pair.sign(message);

    signature.to_bytes()

}

pub fn verify(message: &[u8;32], pub_key: &[u8;32], sig: &[u8;64]) -> bool {

    let public_key: PublicKey = PublicKey::from_bytes(pub_key).unwrap();

    let signature: Signature = Signature::from_bytes(sig).unwrap();

    match public_key.verify(message, &signature) {
        Ok(_) => true,
        Err(_) => false
    }

}