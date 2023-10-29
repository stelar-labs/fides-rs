const K0: u64 = 0x9ae16a3b2f90404f;
const K1: u64 = 0xb492b66fbe98f273;
const K2: u64 = 0xc3a5c85c97cb3127;

pub fn farm_hash_64(data: &[u8]) -> u64 {
    // This function computes a 64-bit FarmHash64 hash of the given data.

    // Check to make sure that the data slice is not empty.
    if data.len() == 0 {
        return 0;
    }

    // Initialize the hash value.
    let mut hash: u64 = 81;

    // Create a temporary buffer to hold the current 64-bit chunk of data.
    let mut tmp_buf: u64 = 0;

    // Iterate over the data, 64 bits at a time.
    let mut off: usize = 0;
    let mut shift: usize = 0;
    while off < data.len() {
        // Load the next 64-bit chunk of data into the temporary buffer.
        tmp_buf |= u64::from(data[off]) << shift;
        shift += 8;
        off += 1;

        // If we have a full 64-bit chunk of data, hash it.
        if shift == 64 {
            // Multiply the temporary buffer by K2.
            tmp_buf = tmp_buf.wrapping_mul(K2);

            // XOR the temporary buffer into the hash value.
            hash ^= tmp_buf;

            // Multiply the hash value by K1 and add K0.
            hash = hash.wrapping_mul(K1).wrapping_add(K0);

            // Clear the temporary buffer and reset the shift counter.
            tmp_buf = 0;
            shift = 0;
        }
    }

    // If we have any remaining data, hash it.
    if shift > 0 {
        // Multiply the temporary buffer by K2.
        tmp_buf = tmp_buf.wrapping_mul(K2);

        // XOR the temporary buffer into the hash value.
        hash ^= tmp_buf;
    }

    // Finalize the hash value.
    hash ^= data.len() as u64;
    hash ^= hash.wrapping_mul(K1);
    hash ^= hash >> 47;
    hash = hash.wrapping_mul(K2);
    hash ^= hash.wrapping_mul(K1);
    hash ^= hash >> 47;

    // Return the final hash value.
    hash
}