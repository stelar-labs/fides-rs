use std::collections::HashMap;

use astro_format::IntoBytes;

use crate::hash::blake_3;

#[derive(Debug,Clone)]
pub struct MerkleTree<T> {
    nodes: HashMap<[u8;32], MerkleNode<T>>,
    root: [u8;32],
    parents: HashMap<[u8; 32], [u8; 32]>,
}

impl<T> MerkleTree<T> where T: IntoBytes + Clone {
    fn update_hash(mut self, mut old_hash: [u8; 32], mut new_hash: [u8; 32]) {

        while let Some(parent_hash) = self.parents.remove(&new_hash) {
            if let Some(mut parent_node) = self.nodes.remove(&parent_hash) {
                // Replace old hash in parent with the new hash
                if let Some(pos) = parent_node.children.iter().position(|&hash| hash == old_hash) {
                    parent_node.children[pos] = new_hash;
                }

                // Get new hash for parent
                let new_parent_hash = parent_node.calculate_hash();

                // Change parent key in nodes
                self.nodes.insert(new_parent_hash, parent_node);

                // Change parent relationships
                if let Some(grandparent_hash) = self.parents.remove(&parent_hash) {
                    self.parents.insert(new_parent_hash, grandparent_hash);
                }

                // Remove old relationships
                self.parents.insert(new_hash, new_parent_hash);

                // Move to the next node up the tree
                old_hash = parent_hash;
                new_hash =new_parent_hash;
            }
        }
    }
    pub fn append(mut self, data: T) {
        // Create a new leaf node
        let new_node = MerkleNode {
            children: vec![],
            data: Some(data),
        };

        // Calculate the hash of the new node
        let new_node_hash = new_node.calculate_hash();

        // Add the new node to the nodes map
        self.nodes.insert(new_node_hash, new_node);

        // Add node hash to parent(if lasts parent has 2 children create a new parent)
        let mut current_right_hash = self.root;

        while let Some(last_child_hash) = self.nodes.get(&current_right_hash).and_then(|node| node.children.last()) {
            current_right_hash = *last_child_hash;
        }

        // If the tree is empty, set the root to the new node
        if current_right_hash == [0; 32] {
            self.root = new_node_hash;
        } else {
            // Add the new node to the rightmost parent or create a new parent
            let parent_hash = self.parents.get(&current_right_hash).cloned().unwrap_or(self.root);
            if let Some(parent_node) = self.nodes.get_mut(&parent_hash) {
                
                if parent_node.children.len() > 1 {
                    // Create a new parent node
                    let new_parent_node: MerkleNode<T> = MerkleNode {
                        children: vec![new_node_hash],
                        data: None,
                    };

                    let new_parent_node_hash = new_parent_node.calculate_hash();
                    // Add new parent node to the tree
                    self.nodes.insert(new_parent_node_hash, new_parent_node);
                    // Add parent relationships
                    self.parents.insert(current_right_hash, new_parent_node_hash);
                    self.parents.insert(new_node_hash, new_parent_node_hash);
                    // Update hashes 
                    self.update_hash(parent_hash, new_parent_node_hash)
                } else {
                    // Add the new node as a child of the current parent node
                    parent_node.children.push(new_node_hash);
                    // Add parent relationship
                    self.parents.insert(new_node_hash, parent_hash);
                    // Recalculate the parent hash
                    let new_parent_hash = parent_node.calculate_hash();
                    // Update hashes
                    self.update_hash(parent_hash, new_parent_hash)
                }
            }
        }
    }
    // pub fn replace(self, index: usize) {
    // }
    // pub fn insert(self, index: usize) {
    // }
    // pub fn remove(self, index: usize) {
    // }
}

#[derive(Debug, Clone)]
pub struct MerkleNode<T> {
    children: Vec<[u8;32]>,
    data: Option<T>
}

impl<T> MerkleNode<T> where T: IntoBytes, {
    pub fn calculate_hash(&self) -> [u8;32] {
        if let Some(ref data) = self.data {
            blake_3(&data.into_bytes())
        } else {
            let mut concatenated_hashes = Vec::new();
            for child in &self.children {
                concatenated_hashes.extend_from_slice(child);
            }
             blake_3(&concatenated_hashes)
        }
    }
}