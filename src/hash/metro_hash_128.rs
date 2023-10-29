pub fn metro_hash_128(data: &[u8]) -> (u64, u64) {
    let mut h0 = 0u64;
    let mut h1 = 0u64;

    for chunk in data.chunks(16) {
        let k0 = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
        let k1 = u64::from_le_bytes(chunk[8..].try_into().unwrap());

        h0 ^= k0;
        h0 = h0.wrapping_mul(0xC83A91E1).wrapping_add(0x52DCE729);
        h1 ^= k1;
        h1 = h1.wrapping_mul(0x8648DBDB).wrapping_add(0x38495AB5);
    }

    h1 ^= h0;

    for byte in data {
        h0 ^= u64::from(*byte);
        h0 = h0.wrapping_mul(5).wrapping_add(0x51E90B6D);
    }

    h1 ^= h0;

    (h0, h1)
}