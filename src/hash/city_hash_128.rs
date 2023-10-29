pub fn city_hash_128(input1: u64, input2: u64) -> (u64, u64) {
    let mul: u64 = 0x9ddfea08eb382d69;
    let hash1: u64 = input1
        .wrapping_mul(mul)
        .rotate_left(32)
        .wrapping_add(input2);
    let hash2: u64 = input2
        .wrapping_mul(mul)
        .rotate_left(32)
        .wrapping_add(hash1);
    (hash1, hash2)
}
