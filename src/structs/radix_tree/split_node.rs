
use astro_format::IntoBytes;
use std::error::Error;
use super::{RadixTree, RadixNode};

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes,
    V: Clone + IntoBytes,
{
    pub fn split_node(&mut self, node_hash: [u8; 32], split_position: usize) -> Result<([u8; 32], [u8; 32]), Box<dyn Error>> {

        if let Some(old_node) = self.nodes.remove(&node_hash) {

            if split_position < old_node.key.len() {
                
                let (left_key_part, right_key_part) = old_node.key.split_at(split_position);

                let right_node = RadixNode {
                    key: right_key_part[1..].to_vec(),
                    children: old_node.children,
                    value: old_node.value,
                };

                let right_node_hash = right_node.hash();

                for child_hash in right_node.children.values() {
                    self.parents.insert(*child_hash, right_node_hash);
                }

                self.nodes.insert(right_node_hash, right_node);
                
                let mut left_node = RadixNode::new();

                left_node.key = left_key_part.to_vec();

                left_node.children.insert(right_key_part[0].clone(), right_node_hash);

                let left_node_hash = left_node.hash();

                self.nodes.insert(left_node_hash, left_node);

                if self.root == node_hash {
                    self.root = left_node_hash;
                }

                // Link the left node to the original node's parent
                if let Some(parent_hash) = self.parents.remove(&node_hash) {
                    self.parents.insert(left_node_hash, parent_hash);
                    if let Some(parent) = self.nodes.get_mut(&parent_hash) {
                        parent.children.insert(left_key_part[0].clone(), left_node_hash);
                    }
                }

                // Set the parent of the right node to be the new left node
                self.parents.insert(right_node_hash, left_node_hash);

                return Ok((left_node_hash, right_node_hash))
            } else {
                Err("Node can't be split")?
            }
        } else {
            Err("Node doesn't exist!")?
        }
    }

}      
