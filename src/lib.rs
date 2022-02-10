use blake3;

pub mod asymmetric;
pub mod symmetric;

pub fn hash(input: &Vec<u8>) -> Vec<u8> {
    blake3::hash(input).as_bytes().to_vec()
}