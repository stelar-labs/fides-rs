pub mod sha_2;
pub mod sha_3;
use blake3;

pub fn blake_3(input: &[u8]) -> [u8; 32] {
    * blake3::hash(input).as_bytes()
}