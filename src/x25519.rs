use rand::{RngCore, rngs::OsRng};
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};

pub fn secret_key() -> [u8; 32] {

    let mut key = [0_u8; 32];

    OsRng.fill_bytes(&mut key);

    let sk: StaticSecret = StaticSecret::from(key);

    sk.to_bytes()

}

pub fn public_key(secret_key: &[u8; 32]) -> [u8; 32] {

    let sk: StaticSecret = StaticSecret::from(*secret_key);

    let pk: PublicKey = PublicKey::from(&sk);

    pk.to_bytes()

}

pub fn shared_key(public_key: &[u8; 32], secret_key: &[u8; 32]) -> [u8; 32] {

    let sk: StaticSecret = StaticSecret::from(*secret_key);

    let pk: PublicKey = PublicKey::from(*public_key);

    let shared_secret: SharedSecret = StaticSecret::diffie_hellman(&sk, &pk);
    
    shared_secret.to_bytes()

}