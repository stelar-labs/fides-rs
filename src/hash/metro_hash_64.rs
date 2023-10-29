const SEED: u64 = 0x0123456789ABCDEF;
const K0: u64 = 0xD6D018F5;
const K1: u64 = 0xA2AA033B;
const K2: u64 = 0x62992FC1;

pub fn metro_hash_64(data: &[u8]) -> u64 {
    let mut hash = SEED;
    let len = data.len();

    let mut offset = 0;

    while offset + 32 <= len {
        let v0 = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);

        let v1 = u64::from_le_bytes([
            data[offset + 8],
            data[offset + 9],
            data[offset + 10],
            data[offset + 11],
            data[offset + 12],
            data[offset + 13],
            data[offset + 14],
            data[offset + 15],
        ]);

        let v2 = u64::from_le_bytes([
            data[offset + 16],
            data[offset + 17],
            data[offset + 18],
            data[offset + 19],
            data[offset + 20],
            data[offset + 21],
            data[offset + 22],
            data[offset + 23],
        ]);

        let v3 = u64::from_le_bytes([
            data[offset + 24],
            data[offset + 25],
            data[offset + 26],
            data[offset + 27],
            data[offset + 28],
            data[offset + 29],
            data[offset + 30],
            data[offset + 31],
        ]);

        hash ^= v0.wrapping_mul(K2);
        hash = hash.rotate_left(29).wrapping_add(v1);
        hash = hash.wrapping_mul(K0);
        hash ^= v2.wrapping_mul(K1);
        hash = hash.rotate_left(29).wrapping_add(v3);
        hash = hash.wrapping_mul(K0);

        offset += 32;
    }

    hash = hash.wrapping_add(SEED);

    while offset + 8 <= len {
        let v = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);

        hash ^= v.wrapping_mul(K2);
        hash = hash.rotate_left(29).wrapping_mul(K0);

        offset += 8;
    }

    if offset + 4 <= len {
        let v = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as u64;

        hash ^= v.wrapping_mul(K2);
        hash = hash.rotate_left(29).wrapping_mul(K0);

        offset += 4;
    }

    while offset < len {
        hash ^= u64::from(data[offset]).wrapping_mul(K0);
        hash = hash.rotate_left(23).wrapping_add(hash.wrapping_mul(K1));

        offset += 1;
    }

    hash ^= hash.rotate_left(37).wrapping_mul(K1);
    hash ^= hash.rotate_left(28);

    hash
}