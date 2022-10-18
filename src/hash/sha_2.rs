use sha2::{Sha224, Sha256, Sha512_224, Sha512_256, Sha384, Sha512, Digest};

pub fn sha_224(input: &[u8]) -> [u8; 28] {
    let mut hasher = Sha224::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_512_224(input: &[u8]) -> [u8; 28] {
    let mut hasher = Sha512_224::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_512_256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_384(input: &[u8]) -> [u8; 48] {
    let mut hasher = Sha384::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

pub fn sha_512(input: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}
