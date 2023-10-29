pub fn murmur_hash3_64(data: &[u8], seed: u64) -> u64 {
    const C1: u64 = 0xff51afd7ed558ccd;
    const C2: u64 = 0xc4ceb9fe1a85ec53;
    const R1: u32 = 31;
    const R2: u32 = 27;
    const M: u64 = 5;
    const N: u64 = 0xe6546b64b7772d8e;

    let mut hash: u64 = seed;
    let mut data_chunks = data.chunks(8);

    for chunk in &mut data_chunks {
        let mut k: u64 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            k |= (byte as u64) << (8 * i as u32);
        }

        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = hash.rotate_left(R2).wrapping_add(hash.wrapping_mul(M)).wrapping_add(N);
    }

    for &byte in data_chunks.next().unwrap_or(&[]) {
        hash ^= (byte as u64).wrapping_mul(C1);
        hash = hash.rotate_left(R1);
        hash = hash.wrapping_mul(C2);
    }

    hash ^= data.len() as u64;
    hash ^= hash.wrapping_shr(33);
    hash = hash.wrapping_mul(0xff51afd7ed558ccd);
    hash ^= hash.wrapping_shr(33);
    hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
    hash ^= hash.wrapping_shr(33);

    hash
}