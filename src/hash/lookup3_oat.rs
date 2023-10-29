const OAT_HASH_SEED: u32 = 0xDEADBEEF;

pub fn lookup3_oat(data: &[u8]) -> u32 {
    let mut hash = OAT_HASH_SEED;

    for &byte in data {
        let byte_value = u32::from(byte);
        hash = hash.wrapping_add(byte_value);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
    }

    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);

    hash
}