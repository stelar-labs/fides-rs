pub fn city_hash_64(input: u64) -> u64 {
    input
        .wrapping_mul(0xc3a5c85c97cb3127)
        .rotate_left(31)
        .wrapping_mul(0x4b7445e07f9b8b22)
}