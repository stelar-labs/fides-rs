pub mod ed25519;
pub mod x25519;
pub mod chacha20poly1305;

use blake3;

pub fn hash(input: &Vec<u8>) -> [u8;32] {
    * blake3::hash(input).as_bytes()
}