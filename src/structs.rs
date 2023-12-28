use opis::Bit;
mod bloom_filter;
mod radix_tree;

#[derive(Debug,Clone)]
pub struct BloomFilter {
    bits: Vec<Bit>,
    hashes: u8
}

pub use crate::structs::radix_tree::RadixTree;