use std::{collections::{HashMap, BTreeMap}, error::Error};

use astro_format::IntoBytes;

use crate::hash::blake_3;

mod insert;
mod rehash;
mod remove;
mod search;
mod split_node;

#[derive(Debug,Clone)]
pub struct RadixTree<K,V> {
    nodes: HashMap<[u8; 32], RadixNode<K,V>>,
    parents: HashMap<[u8;32], [u8;32]>,
    root: [u8; 32],
}

#[derive(Debug,Clone)]
pub struct RadixNode<K,V> {
    children: BTreeMap<K,[u8;32]>,
    key: Vec<K>,
    value: Option<V>,
}

impl<K,V> RadixNode<K,V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes,
    V: IntoBytes,
{
    fn hash(&self) -> [u8; 32] {

        let children_hash = match self.children.iter().next() {
            Some((_, &first_child_hash)) => {
                self.children.iter().skip(1).fold(first_child_hash, |acc, (child_key, &child_hash)| {
                    let child_key_hash = blake_3(&child_key.into_bytes());
                    let combined = [acc, blake_3(&[child_key_hash, child_hash].concat())].concat();
                    blake_3(&combined)
                })
            },
            None => blake_3(&vec![]),
        };
        
        let key_bytes = self.key.iter().flat_map(|k| k.clone().into_bytes()).collect::<Vec<u8>>();
        let key_hash = blake_3(&key_bytes);

        let children_key_concat = [children_hash, key_hash].concat();
        let children_key_hash = blake_3(&children_key_concat);

        let value_bytes = match &self.value {
            Some(v) => v.into_bytes(),
            None => Vec::new(),
        };
        let value_hash = blake_3(&value_bytes);

        let children_key_value_concat = [children_key_hash, value_hash].concat();
        blake_3(&children_key_value_concat)

    }

    fn new() -> Self {
        RadixNode {
            children: BTreeMap::new(),
            key: vec![],
            value: None,
        }
    }

}

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Display + std::fmt::Debug,
    V: Clone + IntoBytes + std::fmt::Debug,
{
    // Constructor
    pub fn new() -> Self {
        RadixTree {
            nodes: HashMap::new(),
            parents: HashMap::new(),
            root: [0;32], 
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree: RadixTree<String, i32> = RadixTree::new();
        assert!(tree.nodes.is_empty());
        assert!(tree.parents.is_empty());
        // Check if the root is initialized correctly if applicable
    }

    #[test]
    fn test_insert() {
        let mut tree = RadixTree::new();
        tree.insert("key1".as_bytes(), 1);
        assert_eq!(tree.search("key1".as_bytes()), Some(&1));
        // Test inserting a key that partially overlaps with an existing key
        // Test inserting a key that fully overlaps with an existing key
    }

    #[test]
    fn test_remove() {
        let mut tree = RadixTree::new();
        println!("tree -> {:?}", tree);
        tree.insert("key1".chars(), 1);
        println!("tree -> {:?}", tree);
        assert!(tree.remove("key1".chars()).is_ok());
        assert_eq!(tree.search("key1".chars()), None);
        // Test removing a non-existent key
        // Test removing a key that affects the structure of the tree
    }

    #[test]
    fn test_search() {
        let mut tree = RadixTree::new();
        tree.insert("key1".as_bytes(), 1);
        assert_eq!(tree.search("key1".as_bytes()), Some(&1));
        assert_eq!(tree.search("nonexistent".as_bytes()), None);
        // Test searching for a key that partially matches an existing key
    }

    #[test]
    fn test_node_splitting_with_chars() {
        
        let mut tree1 = RadixTree::new();

        tree1.insert("abcde".chars(), 1);
        
        tree1.insert("abcfg".chars(), 2);
        
        // Verify that both keys are present and return correct values
        assert_eq!(tree1.search("abcde".chars()), Some(&1));
        assert_eq!(tree1.search("abcfg".chars()), Some(&2));

        // Optionally, verify the absence of a key that should not exist
        assert_eq!(tree1.search("abcf".chars()), None);

        // Additional checks can be made based on the internal structure of your RadixTree
        assert!(tree1.remove("abcde".chars()).is_ok());

        assert_eq!(tree1.search("abcde".chars()), None);
        
        assert_eq!(tree1.search("abcfg".chars()), Some(&2));

        let mut tree2 = RadixTree::new();

        tree2.insert("abcfg".chars(), 2);

        assert_eq!(tree1.root, tree2.root);
    
    }

}

