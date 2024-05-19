use opis::Bit;
pub mod bloom_filter;
pub mod merkle_tree;
pub mod radix_tree;

#[derive(Debug,Clone)]
pub struct BloomFilter {
    bits: Vec<Bit>,
    hashes: u8
}

pub use crate::structs::radix_tree::RadixTree;
pub use crate::structs::merkle_tree::MerkleTree;