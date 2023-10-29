pub fn spooky_hash_v2(data: &[u8]) -> u64 {
    
    const SC_MAGIC: u64 = 0xdeadbeef_deadbeef;
    const SC_C0: u64 = 0x104c11db7;
    const SC_C1: u64 = 0xc4ce7d5b;

    let mut a = SC_MAGIC;
    let mut b = SC_MAGIC;
    let mut c = SC_C0;
    let mut d = SC_C1;

    let len = data.len();

    let mut i = 0;

    while i + 8 <= len {
        let k = u64::from_ne_bytes([
            data[i], data[i + 1], data[i + 2], data[i + 3], data[i + 4], data[i + 5], data[i + 6], data[i + 7],
        ]);
        i += 8;

        a ^= k;
        a = a.rotate_left(32);
        a = a.wrapping_add(b);
        c ^= a;
        d ^= c;
        b = b.rotate_left(24);
        b = b.wrapping_add(d);
    }

    if i < len {
        let mut k: u64 = 0;
        for shift in (0..64).step_by(8) {
            if i < len {
                k |= u64::from(data[i]) << shift;
                i += 1;
            }
        }

        a ^= k;
        a = a.rotate_left(32);
        a = a.wrapping_add(b);
    }

    c = c.wrapping_add(d);
    b = b.wrapping_add(c);

    // Short mix
    a ^= b;
    c = c.rotate_left(40);
    a = a.wrapping_add(c);
    b = b.rotate_left(48);
    a = a.wrapping_add(b);

    // Finalization
    let mut c = SC_MAGIC;
    let mut d = SC_MAGIC;
    c ^= a;
    d ^= b;
    a = a.rotate_left(32);
    a = a.wrapping_add(b);
    c = c.wrapping_add(a);
    d = d.wrapping_add(c);

    a = a.rotate_left(24);
    a = a.wrapping_add(b);
    c = c.wrapping_add(a);
    d = d.wrapping_add(c);

    (d << 32) | c
}