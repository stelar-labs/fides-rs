const YSC_HASH_SEED: u32 = 0xC3D2E1F0;

pub fn lookup3_ycs(data: &[u8]) -> u32 {
    let mut hash = YSC_HASH_SEED;

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

fn main() {
    let data = b"Hello, Lookup3ycs (Yea-saying-Critter Seed)!";
    let hash = lookup3_ycs(data);
    println!("Lookup3ycs: {:x}", hash);
}
