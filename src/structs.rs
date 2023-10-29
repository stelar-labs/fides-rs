use opis::Bit;
mod merkle_tree;
mod bloom_filter;

#[derive(Debug,Clone)]
pub struct BloomFilter {
    bits: Vec<Bit>,
    hashes: u8
}