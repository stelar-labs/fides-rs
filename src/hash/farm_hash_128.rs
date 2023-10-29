pub fn farm_hash_128(data: &[u8]) -> (u64, u64) {
    let mut seed = [0; 2];
    let mut state = [0; 2];
    let mut offset = 0;

    while offset < data.len() {
        let mut chunk = [0; 16];
        let chunk_len = std::cmp::min(data.len() - offset, chunk.len());
        chunk[0..chunk_len].copy_from_slice(&data[offset..offset + chunk_len]);
        offset += chunk_len;

        farm_hash_update(&seed, &mut state, &chunk);
    }

    (state[0], state[1])
}

fn farm_hash_update(seed: &[u64; 2], state: &mut [u64; 2], chunk: &[u8; 16]) {
    let mut mul = seed[0] ^ seed[1];
    mul *= 0x9e3779b9_7f4a7c15;
    mul ^= mul >> 29;
    mul *= 0x85ebca6b_16a0911b;

    state[0] ^= mul;
    state[0] ^= chunk[0] as u64 | (chunk[1] as u64) << 8 | (chunk[2] as u64) << 16 | (chunk[3] as u64) << 24;
    state[1] ^= mul;
    state[1] ^= chunk[4] as u64 | (chunk[5] as u64) << 8 | (chunk[6] as u64) << 16 | (chunk[7] as u64) << 24;

    farm_hash_mix(state);
}

fn farm_hash_mix(state: &mut [u64; 2]) {
    state[0] ^= state[1];
    state[0] = state[0].wrapping_mul(0x85ebca6b_16a0911b);
    state[0] ^= state[0] >> 29;
    state[1] ^= state[0];
    state[1] = state[1].wrapping_mul(0x9e3779b9_7f4a7c15);
    state[1] ^= state[1] >> 29;
}