pub fn pearson(data: &[u8]) -> u64 {
    let mut h = 0u64;

    for byte in data {
        h ^= u64::from(*byte);
        h = h.wrapping_mul(0x8088405);
    }

    h ^= h >> 31;
    h = h.wrapping_mul(0x8088405);
    h ^= h >> 31;

    h
}