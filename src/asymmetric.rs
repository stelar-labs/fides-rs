use rand::{RngCore, rngs::OsRng};

pub fn private_key() -> [u8;32] {
    let mut key = [0_u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn public_key(_priv_key: &[u8;32]) -> [u8;32] {
    [0_u8;32]
}

pub fn shared_key(_priv_key: &[u8;32], _pub_key: &[u8;32]) -> [u8;32] {
    [0_u8;32]
}

pub fn sign() {}

pub fn verify() {}