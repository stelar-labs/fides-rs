use std::collections::{HashMap, BTreeMap};

pub struct RadixTree<K, V> {
    root: [u8; 32],
    nodes: HashMap<[u8; 32], RadixNode<K, V>>,
    // parents: HashMap<[u8;32], [u8;32]>,
}

pub struct RadixNode<K, V> {
    key: Option<Vec<K>>,
    children: BTreeMap<K, [u8; 32]>,
    value: V,
}

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord,
    V: Clone,
{
    // Constructor
    pub fn new() -> Self {
        RadixTree {
            root: [0; 32],
            nodes: HashMap::new(),
        }
    }
    // Insert method
    // pub fn insert(&mut self, key: Vec<K>, value: V) -> Result<(), Box<dyn Error>> {
    //     // Implementation for inserting a key-value pair
    //     Ok(())
    // }

    // // Remove method
    // pub fn remove(&mut self, key: Vec<K>) -> Result<(), Box<dyn Error>> {
    //     // Implementation for removing a key
    //     Ok(())
    // }

    // Search method
    pub fn search(&self, search_key: &[K]) -> Option<&V> {
        let mut current_node_hash = &self.root;
        let mut search_index = 0;

        while search_index < search_key.len() {
            if let Some(node) = self.nodes.get(current_node_hash) {
                match &node.key {
                    Some(node_key) => {
                        // Compressed key case
                        for (i, key_part) in node_key.iter().enumerate() {
                            if search_index + i >= search_key.len() || key_part != &search_key[search_index + i] {
                                // Mismatch or search_key is shorter than node_key
                                return None;
                            }
                        }

                        // Update search index after matching with compressed key
                        search_index += node_key.len();

                        // If it's a complete match of the search key
                        if search_index == search_key.len() {
                            return Some(&node.value);
                        }

                        // Continue with children if there's more to search
                        if let Some(next_node_hash) = node.children.get(&search_key[search_index]) {
                            current_node_hash = next_node_hash;
                            search_index += 1; // Move to the next part of the search key
                            continue;
                        } else {
                            return None;
                        }
                    },
                    None => {
                        // Non-compressed key case
                        if let Some(next_node_hash) = node.children.get(&search_key[search_index]) {
                            current_node_hash = next_node_hash;
                            search_index += 1; // Move to the next part of the search key
                            continue;
                        } else {
                            return None;
                        }
                    }
                }
            } else {
                // Node not found
                return None;
            }
        }

        None
    }
}