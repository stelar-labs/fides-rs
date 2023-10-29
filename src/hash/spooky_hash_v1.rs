pub fn spooky_hash_v1(data: &[u8]) -> u64 {
    const SC1: u64 = 0x104c11db7;
    const SC2: u64 = 0xc4ce7d5b;
    let mut a: u64 = 0xdeadbeef_deadbeef;
    let mut b: u64 = 0xdeadbeef_deadbeef;

    for &byte in data {
        a = a.wrapping_add(byte as u64);
        b = b.wrapping_add(a);
    }

    let mut result: u64 = SC1.wrapping_sub(b).wrapping_sub(a);
    result ^= a >> 37;
    result ^= b >> 42;
    result ^= a << 24;
    result ^= b << 16;

    result
}