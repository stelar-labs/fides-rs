use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512, Digest};

pub fn sha_224(input: &[u8]) -> [u8; 28] {
    let mut hasher = Sha3_224::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_384(input: &[u8]) -> [u8; 48] {
    let mut hasher = Sha3_384::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_512(input: &[u8]) -> [u8; 64] {
    let mut hasher = Sha3_512::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}