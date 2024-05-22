use std::collections::HashMap;

use astro_format::{IntoBytes, TryFromBytes};

use crate::hash::blake_3;

#[derive(Debug,Clone)]
pub struct MerkleTree<T> {
    height: usize,
    nodes: HashMap<[u8;32], MerkleNode<T>>,
    parents: HashMap<[u8;32], [u8;32]>,
    root: [u8;32],
    width: usize,
}

impl<T> IntoBytes for MerkleTree<T>
where
    T: IntoBytes
{
    fn into_bytes(&self) -> Vec<u8> {
        // Create a result vector starting with the root
        let mut result = Vec::new();
        result.extend_from_slice(&self.root);

        // Append height bytes as u64 in little endian
        result.extend_from_slice(&(self.height as u64).to_le_bytes());

        // Append width bytes as u64 in little endian
        result.extend_from_slice(&(self.width as u64).to_le_bytes());

        // Create nodes bytes
        let nodes_iter = self.nodes.iter().map(|(hash, node)| {
            let mut node_bytes = Vec::new();
            node_bytes.extend_from_slice(hash);
            node_bytes.extend_from_slice(&node.into_bytes());
            node_bytes
        });
        let nodes_bytes = astro_format::encode(nodes_iter).unwrap_or_else(|_| Vec::new());

        // Create parents bytes
        let parents_iter = self.parents.iter().map(|(child_hash, parent_hash)| {
            let mut parent_bytes = Vec::new();
            parent_bytes.extend_from_slice(child_hash);
            parent_bytes.extend_from_slice(parent_hash);
            parent_bytes
        });
        let parents_bytes = astro_format::encode(parents_iter).unwrap_or_else(|_| Vec::new());

        // Append encoded nodes and parents bytes to the result
        result.extend_from_slice(&nodes_bytes);
        result.extend_from_slice(&parents_bytes);

        result
    }
}

impl<'a, T> TryFromBytes<'a> for MerkleTree<T>
where
    T: TryFromBytes<'a>
{
    fn try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut offset = 0;
        // Extract root
        let mut root = [0u8; 32];
        root.copy_from_slice(&value[offset..offset + 32]);
        offset += 32;

        // Extract height
        let height = u64::from_le_bytes(value[offset..offset + 8].try_into()?) as usize;
        offset += 8;

        // Extract width
        let width = u64::from_le_bytes(value[offset..offset + 8].try_into()?) as usize;
        offset += 8;

        // Use decode to get the nodes and parents buffer
        let buffers: Vec<&[u8]> = astro_format::decode(&value[offset..])?;
        let nodes_buffer = buffers.get(0).ok_or("missing nodes buffer")?;
        let parents_buffer = buffers.get(1).ok_or("missing parents buffer")?;
        // Decode the nodes using astro_format
        let nodes_bytes: Vec<&[u8]> = astro_format::decode(nodes_buffer)?;
        let mut nodes = HashMap::new();
        for node_bytes in nodes_bytes {
            let hash = {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&node_bytes[0..32]);
                hash
            };
            let node = MerkleNode::<T>::try_from_bytes(&node_bytes[32..])?;
            nodes.insert(hash, node);
        }
        // Decode parents
        let parents_bytes: Vec<&[u8]> = astro_format::decode(parents_buffer)?;
        let mut parents = HashMap::new();
        for parent_bytes in parents_bytes {
            let child_hash = {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&parent_bytes[0..32]);
                hash
            };
            let parent_hash = {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&parent_bytes[32..64]);
                hash
            };
            parents.insert(child_hash, parent_hash);
        }
        Ok(MerkleTree { height, nodes, root, parents, width })
    }
}

impl<T> MerkleTree<T> where T: IntoBytes + Clone {
    pub fn new() -> Self {
        MerkleTree {
            height: 0,
            nodes: HashMap::new(),
            parents: HashMap::new(),
            root: [0u8;32],
            width: 0,
        }
    }
    pub fn hash(&self) -> [u8;32] {
        self.root
    }
    fn update_hash(&mut self, mut old_hash: [u8; 32], mut new_hash: [u8; 32]) {
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
                new_hash = new_parent_hash;
            }
        }
        // Update root if it was changed
        if old_hash == self.root {
            self.root = new_hash;
        }
    }
    pub fn append(&mut self, data: T) {
        // Create a new leaf node
        let new_node = MerkleNode {
            children: vec![],
            data: Some(data),
        };
        // Calculate the hash of the new node
        let new_node_hash = new_node.calculate_hash();
        // Add the new node to the nodes map
        self.nodes.insert(new_node_hash, new_node);

        // If the tree is empty, set the root to the new node
        if self.width == 0 {
            let new_root = MerkleNode {
                children: vec![new_node_hash],
                data: None,
            };
            let new_root_hash = new_root.calculate_hash();
            self.nodes.insert(new_root_hash, new_root);
            // Add new root as the parent of the new node
            self.parents.insert(new_node_hash, new_root_hash);
            // Set the new root
            self.root = new_root_hash;
            // Increase height
            self.height += 1
        } else {
            // If the width equals 2^height:
            if self.width == (1 << self.height) {
                // Increase height
                self.height += 1;

                // Create a new root node
                let old_root = self.root;
                let mut new_root = MerkleNode {
                    children: vec![old_root],
                    data: None,
                };

                // Create the second root branch
                let mut current_hash = new_node_hash;

                // Start from the bottom with the new node and climb up to the root
                for _ in 0..(self.height - 1) {
                    let intermediate_node = MerkleNode {
                        children: vec![current_hash],
                        data: None,
                    };
                    let intermediate_hash = intermediate_node.calculate_hash();
                    self.nodes.insert(intermediate_hash, intermediate_node);
                    self.parents.insert(current_hash, intermediate_hash);
                    current_hash = intermediate_hash;
                }

                new_root.children.push(current_hash);
                let new_root_hash = new_root.calculate_hash();
                self.nodes.insert(new_root_hash, new_root);

                // Add new root as the parent of the old root
                self.parents.insert(old_root, new_root_hash);

                // Add new root as the parent of the second branch
                self.parents.insert(current_hash, new_root_hash);

                // Set the new root
                self.root = new_root_hash;
            } else {
                let mut current_right_hash = self.root;
                // Find the right most end node
                while let Some(last_child_hash) = self.nodes.get(&current_right_hash).and_then(|node| node.children.last()) {
                    current_right_hash = *last_child_hash;
                }
                // Add the new node to the rightmost parent
                let parent_hash = self.parents.get(&current_right_hash).cloned().unwrap_or(self.root);
                if let Some(parent_node) = self.nodes.get_mut(&parent_hash) {
                    // Add the new node as a child of the current parent node
                    parent_node.children.push(new_node_hash);
                    // Add parent relationship
                    self.parents.insert(new_node_hash, parent_hash);
                    // Recalculate the parent hash
                    let new_parent_hash = parent_node.calculate_hash();
                    // Update hashes
                    self.update_hash(parent_hash, new_parent_hash);
                }
            }
        }
        self.width += 1;
    }
    pub fn replace(mut self, index: usize, data: T) {
        // traverse the tree to find the hash of the old node
        let height = self.height;
        // let mut lowest_hash = self.root;

        // while let Some(child_node) = self.nodes.get(&lowest_hash) {
        //     match child_node.children.first() {
        //         Some(res) => {
        //             lowest_hash = *res;
        //             height += 1;
        //         },
        //         None => break,
        //     }
        // }
        // find the old hash using the height and index
        let mut old_hash = self.root;
        let mut idx = index;
        let mut range_start = 0;
        let mut range_end = 2usize.pow(height as u32);

        for _ in 0..height {
            let mid = (range_start + range_end) / 2;
            if idx < mid {
                range_end = mid;
            } else {
                range_start = mid;
                idx -= mid;
            }

            if let Some(current_node) = self.nodes.get(&old_hash) {
                old_hash = current_node.children[(idx >= mid) as usize];
            }
        }
        // create a new node
        let new_node: MerkleNode<T> = MerkleNode {
            data: Some(data.clone()),
            children: vec![]
        };
        // calculate the hash of the new node
        let new_hash = new_node.calculate_hash();
        // add the new node to the tree
        self.nodes.insert(new_hash, new_node);
        // get the parent of the old node with remove and update the parents
        match self.parents.remove(&old_hash) {
            Some(parent_hash) => {
                self.parents.insert(new_hash, parent_hash);
            },
            None => todo!(),
        }
        // update the tree hash
        self.update_hash(old_hash, new_hash);
    }
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

impl<T> IntoBytes for MerkleNode<T>
where
    T: IntoBytes
{
    fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Determine data flag byte
        let flag: u8 = if self.data.is_some() { 1 } else { 0 };
        bytes.push(flag);

        // Append data bytes or joined children hashes
        if let Some(ref data) = self.data {
            bytes.extend_from_slice(&data.into_bytes());
        } else {
            for child_hash in &self.children {
                bytes.extend_from_slice(child_hash);
            }
        }

        bytes
    }
}

impl<'a, T> TryFromBytes<'a> for MerkleNode<T>
where
    T: TryFromBytes<'a>
{
    fn try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if value.is_empty() {
            return Err("Input bytes are empty".into());
        }
        // Read the flag byte
        let flag = value[0];
        let mut offset = 1;
        // Parse data based on the flag
        let data = if flag == 1 {
            let data = T::try_from_bytes(&value[offset..])?;
            Some(data)
        } else {
            None
        };
        // Parse children hashes if flag is 0
        let children = if flag == 0 {
            let mut children = Vec::new();
            while offset + 32 <= value.len() {
                let mut hash = [0u8; 32];
                hash.copy_from_slice(&value[offset..offset + 32]);
                children.push(hash);
                offset += 32;
            }
            children
        } else {
            Vec::new()
        };
        Ok(MerkleNode { children, data })
    }
}