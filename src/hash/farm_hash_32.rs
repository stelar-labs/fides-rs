// The FarmHash32 constants
const K0: u32 = 0xc2b2ae35;
const K1: u32 = 0x299bf731;
const K2: u32 = 0x9e3779b9;

pub fn farm_hash_32(data: &[u8]) -> u32 {
    // This function computes a 32-bit FarmHash32 hash of the given data.

    // Check to make sure that the data slice is not empty.
    if data.len() == 0 {
        return 0;
    }

    // Initialize the hash value.
    let mut hash: u32 = 0;

    // Create a temporary buffer to hold the current 32-bit chunk of data.
    let mut tmp_buf: u32 = 0;

    // Iterate over the data, 32 bits at a time.
    let mut off: usize = 0;
    let mut shift: usize = 0;
    while off < data.len() {
        // Load the next 32-bit chunk of data into the temporary buffer.
        tmp_buf |= u32::from(data[off]) << shift;
        shift += 8;
        off += 1;

        // If we have a full 32-bit chunk of data, hash it.
        if shift == 32 {
            // Multiply the temporary buffer by K0.
            tmp_buf = tmp_buf.wrapping_mul(K0);

            // Rotate the temporary buffer left by 11 bits.
            tmp_buf = tmp_buf.rotate_left(11);

            // Multiply the temporary buffer by K1.
            tmp_buf = tmp_buf.wrapping_mul(K1);

            // XOR the temporary buffer into the hash value.
            hash ^= tmp_buf;

            // Multiply the hash value by 5 and add K2.
            hash = hash.wrapping_mul(5).wrapping_add(K2);

            // Clear the temporary buffer and reset the shift counter.
            tmp_buf = 0;
            shift = 0;
        }
    }

    // If we have any remaining data, hash it.
    if shift > 0 {
        // Multiply the temporary buffer by K0.
        tmp_buf = tmp_buf.wrapping_mul(K0);

        // Rotate the temporary buffer left by 11 bits.
        tmp_buf = tmp_buf.rotate_left(11);

        // Multiply the temporary buffer by K1.
        tmp_buf = tmp_buf.wrapping_mul(K1);

        // XOR the temporary buffer into the hash value.
        hash ^= tmp_buf;
    }

    // Finalize the hash value.
    hash ^= data.len() as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    // Return the final hash value.
    hash
}