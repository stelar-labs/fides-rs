use crate::hash;
use rand::{RngCore, rngs::OsRng};
use x25519_dalek::{StaticSecret, PublicKey, x25519};

pub fn private_key() -> [u8; 32] {

    let mut key = [0_u8; 32];

    OsRng.fill_bytes(&mut key);

    let private_key: StaticSecret = StaticSecret::from(key);

    private_key.to_bytes()

}

pub fn public_key(priv_key: &[u8; 32]) -> [u8; 32] {

    let private_key: StaticSecret = StaticSecret::from(*priv_key);

    let public_key: PublicKey = PublicKey::from(&private_key);

    public_key.to_bytes()

}

pub fn shared_key(priv_key: &[u8; 32], pub_key: &[u8; 32]) -> [u8; 32] {

    let shared_point: [u8; 32] = x25519(*priv_key, *pub_key);

    hash(&shared_point[..])

}