pub mod asymmetric;
pub mod symmetric;

use blake3;

pub fn hash(input: &Vec<u8>) -> [u8;32] {
    * blake3::hash(input).as_bytes()
}