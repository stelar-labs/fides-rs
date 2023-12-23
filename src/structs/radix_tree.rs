use std::collections::{HashMap, BTreeMap};

pub struct RadixTree<K, V> {
    root: [u8; 32],
    nodes: HashMap<[u8; 32], RadixNode<K, V>>,
    parents: HashMap<[u8;32], [u8;32]>,
}

pub struct RadixNode<K, V> {
    key: Option<Vec<K>>,
    children: BTreeMap<K, [u8; 32]>,
    value: Option<V>,
}

impl<K, V> RadixNode<K, V>
{
    fn hash_node(&self) -> [u8; 32] {
        [0; 32]
    }
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
            parents: HashMap::new()
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
                            return node.value.as_ref();
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

    fn split_node(&mut self, node_hash: [u8; 32], split_position: usize) {
        if let Some(old_node) = self.nodes.remove(&node_hash) {
            if let Some(node_key) = old_node.key.clone() {
                let (left_key_part, right_key_part) = node_key.split_at(split_position);
    
                // Create new right node first
                let right_node = RadixNode {
                    key: Some(right_key_part[1..].to_vec()), // Exclude the split character
                    children: old_node.children, // Old node's children are now right node's children
                    value: old_node.value, // Move the value to the right node
                };

                let right_node_hash = right_node.hash_node(); // Generate hash for the right node

                for child_hash in right_node.children.values() {
                    self.parents.insert(*child_hash, right_node_hash);
                }
    
                self.nodes.insert(right_node_hash, right_node); // Insert the right node into the tree

                
                // Create new left node
                let left_node = RadixNode {
                    key: Some(left_key_part.to_vec()),
                    children: {
                        let mut map = BTreeMap::new();
                        map.insert(right_key_part[0].clone(), right_node_hash); // Point to the right node
                        map
                    },
                    value: None, // Left node might not carry the value
                };
                let left_node_hash = left_node.hash_node(); // Generate hash for the left node
                self.nodes.insert(left_node_hash, left_node); // Insert the left node into the tree
    
                // Link the left node to the original node's parent
                if let Some(parent_hash) = self.parents.remove(&node_hash) {
                    self.parents.insert(left_node_hash, parent_hash);
                    if let Some(parent) = self.nodes.get_mut(&parent_hash) {
                        parent.children.insert(left_key_part[0].clone(), left_node_hash);
                    }
                }
    
                // Set the parent of the right node to be the new left node
                self.parents.insert(right_node_hash, left_node_hash);
            }
        }
    }        

}
