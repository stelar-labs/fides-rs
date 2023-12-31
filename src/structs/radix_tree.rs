use std::{collections::{HashMap, BTreeMap}, error::Error};

use astro_format::IntoBytes;

use crate::hash::blake_3;

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
                self.children.iter().skip(1).fold(first_child_hash, |acc, (_, &child_hash)| {
                    let combined = [acc, child_hash].concat();
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
    V: Clone + IntoBytes,
{
    // Constructor
    pub fn new() -> Self {
        RadixTree {
            nodes: HashMap::new(),
            parents: HashMap::new(),
            root: [0;32], 
        }
    }

    // Insert method
    pub fn insert<I>(&mut self, key: I, value: V)
    where I: IntoIterator<Item = K>, {

        let mut current_node_hash = self.root;

        if self.root == [0;32] {

            let key_vec: Vec<K> = key.into_iter().collect();

            let new_node = RadixNode {
                children: BTreeMap::new(),
                key: key_vec,
                value: Some(value.clone()),
            };

            let new_node_hash = new_node.hash();
            
            self.nodes.insert(new_node_hash, new_node);

            self.root = new_node_hash

        } else {

            let mut key_parts = key.into_iter().peekable();

            while let Some(current_key_part) = key_parts.next() {

                let is_last_k = key_parts.peek().is_none();

                let current_node_key_opt = match self.nodes.get(&current_node_hash) {
                    Some(current_node) => Some(current_node.key.clone()),
                    None => None,
                };

                if let Some(current_node_key) = current_node_key_opt {

                    if is_last_k {

                        if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                            current_node.value = Some(value);
                        }

                        self.rehash(&current_node_hash);
                        
                        return;

                    } else {

                        let mut split_position = 0;
                        let mut matched = true;
                        
                        for ck in current_node_key.iter() {
                            if let Some(next_key_part) = key_parts.next() {
                                if next_key_part != *ck {
                                    matched = false;
                                    break;
                                }
                                split_position += 1;
                            } else {
                                matched = false;
                                break;
                            }
                        }

                        if matched {

                            if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                                current_node.value = Some(value);
                            }

                            self.rehash(&current_node_hash);
                                
                            return;

                        } else if split_position != 0 {

                            match self.split_node(current_node_hash, split_position) {
                                Ok((left_node_hash, _right_node_hash)) => {
                                    current_node_hash = left_node_hash;
                                },
                                Err(e) => {
                                    eprintln!("{}", e);
                                    return;
                                }
                            }

                        } else {

                            let mut new_node_key = vec![];
                            
                            if !is_last_k {
                                while let Some(key_part) = key_parts.next() {
                                    new_node_key.push(key_part);
                                }
                            };

                            let new_node = RadixNode {
                                children: BTreeMap::new(),
                                key: new_node_key,
                                value: Some(value.clone()),
                            };

                            let new_node_hash = new_node.hash();
                            
                            self.nodes.insert(new_node_hash, new_node);

                            if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                                current_node.children.insert(current_key_part, new_node_hash);
                            }

                            self.parents.insert(new_node_hash, current_node_hash);

                            self.rehash(&current_node_hash);

                        }

                    }

                }

            }
        
        }

    }

    
    // Remove method
    pub fn remove<I>(&mut self, key: I) -> Result<(), Box<dyn Error>> where I: IntoIterator<Item = K>, {
        
        let mut current_node_hash = self.root;
        
        let mut key_parts = key.into_iter().peekable();

        while let Some(current_key_part) = key_parts.next() {
            
            let is_last_k = key_parts.peek().is_none();

            if let Some(current_node) = self.nodes.get(&current_node_hash) {

                if current_node.key.is_empty() {

                    if is_last_k {

                        if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                            current_node.value = None;
                        }

                        self.rehash(&current_node_hash);

                        return Ok(());

                    } else {

                        match current_node.children.get(&current_key_part) {
                            Some(next_node_hash) => {
                                current_node_hash = *next_node_hash;
                            },
                            None => return Ok(()),
                        }

                    }

                } else {

                    let mut key_matched = true;
                    let mut key_iter = current_node.key.iter();

                    while let Some(ck) = key_iter.next() {
                        if Some(ck) != key_parts.next().as_ref() {
                            key_matched = false;
                            break;
                        }
                    }

                    if key_matched && key_iter.next().is_none() && is_last_k {
                        if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                            current_node.value = None;
                        }
                        self.rehash(&current_node_hash);
                        return Ok(());
                    
                    } else if key_matched {

                        if let Some(next_node_hash) = current_node.children.get(&current_key_part) {
                            current_node_hash = *next_node_hash;
                        } else {
                            return Err("Key does not match!".into());
                        }
                    
                    } else {
                        return Err("Key does not match!".into());
                    }

                }

            } else {
                return Err("Couldn't find node!")?
            }

        }

        Ok(())
    }

    pub fn search<I>(&self, key: I) -> Option<&V>
    where
        I: IntoIterator<Item = K>,
    {
        let mut current_node_hash = &self.root;
        let mut key_parts = key.into_iter().peekable();

        while let Some(current_key_part) = key_parts.next() {
            if let Some(node) = self.nodes.get(current_node_hash) {
                if node.key.is_empty() {
                    if let Some(next_node_hash) = node.children.get(&current_key_part) {
                        current_node_hash = next_node_hash;
                        continue;
                    } else {
                        return None;
                    }
                } else {

                    let mut key_matched = true;

                    let mut key_iter = node.key.iter();

                    if Some(&current_key_part) != key_iter.next() {
                        return None;
                    }

                    while let Some(ck) = key_iter.next() {
                        if Some(ck) != key_parts.next().as_ref() {
                            key_matched = false;
                            break;
                        }
                    }

                    if key_matched && key_iter.next().is_none() && key_parts.peek().is_none() {
                        // The entire key matches and there are no more parts
                        return node.value.as_ref();
                    } else if key_matched {
                        // The key matches so far, continue with the next node
                        if let Some(next_node_hash) = node.children.get(&key_parts.peek().unwrap_or(&&current_key_part)) {
                            current_node_hash = next_node_hash;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }

        None
    }

    fn split_node(&mut self, node_hash: [u8; 32], split_position: usize) -> Result<([u8; 32], [u8; 32]), Box<dyn Error>> {
        
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

    fn rehash(&mut self, child_hash: &[u8; 32]) {

        let (new_child_hash, parent_hash_opt) = if let Some(child_node) = self.nodes.remove(child_hash) {
            
            let new_child_hash = child_node.hash();
    
            self.nodes.insert(new_child_hash, child_node);

            let parent_hash_opt = self.parents.get(child_hash).cloned();
    
            (new_child_hash, parent_hash_opt)

        } else {

            return;

        };

        if let Some(parent_hash) = parent_hash_opt {

            if let Some(mut parent_node) = self.nodes.remove(&parent_hash) {

                if let Some((child_key, _)) = parent_node.children.iter().find(|(_,v)| *v == child_hash) {

                    parent_node.children.insert(child_key.clone(), new_child_hash);

                }
    
                self.nodes.insert(parent_hash, parent_node);

            }
    
            self.rehash(&parent_hash);

        }

    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = RadixTree::new();
        println!("tree.new -> {:?}", tree);
        let key = vec![0_u8, 1];
        let value = "value";

        tree.insert(key.clone(), value);
        println!("tree.insert -> {:?}", tree);

        assert_eq!(tree.search(key), Some(&value));
    }

}
