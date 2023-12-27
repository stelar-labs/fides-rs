use std::{collections::{HashMap, BTreeMap}, error::Error};

pub struct RadixTree<K,V> {
    root: [u8; 32], nodes: HashMap<[u8; 32], RadixNode<K,V>>, parents: HashMap<[u8;32], [u8;32]>,
}

#[derive(Clone)]
pub struct RadixNode<K,V> {
    key: Vec<K>, children: BTreeMap<K,[u8; 32]>, value: Option<V>,
}

impl<K, V> RadixNode<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord,
{
    fn hash(&self) -> [u8; 32] {
        [0; 32]
    }

    fn new() -> Self {
        RadixNode {
            key: vec![], children: BTreeMap::new(), value: None,
        }
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
            root: [0; 32], nodes: HashMap::new(), parents: HashMap::new()
        }
    }

    // Insert method
    pub fn insert(&mut self, key: Vec<K>, value: V) {

        let mut i = 0;
        let last_i = key.len() - 1;
        
        let mut current_node_hash = self.root;

        while i < key.len() {

            let current_node_key_opt = match self.nodes.get(&current_node_hash) {
                Some(current_node) => Some(current_node.key.clone()),
                None => None,
            };

            if let Some(current_node_key) = current_node_key_opt {

                if i == last_i {

                    if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                        current_node.value = Some(value);
                    }

                    self.rehash(&current_node_hash);
                    
                    return;

                } else {

                    if current_node_key == &key[i..] {

                        if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                            current_node.value = Some(value);
                        }

                        self.rehash(&current_node_hash);
                            
                        return;
                    
                    } else {
                        let mut split_position = 0;

                        for (j, ck) in current_node_key.iter().enumerate() {

                            if key[i] == *ck {
                                i += 1
                            } else {
                                split_position = j;
                                break;
                            }
                        
                        }
                        
                        if split_position != 0 {

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

                            let new_node_key = if i == last_i {
                                vec![]
                            } else {
                                key[i + 1..].to_vec()
                            };
    
                            let new_node = RadixNode {
                                key: new_node_key,
                                children: BTreeMap::new(),
                                value: Some(value.clone()),
                            };
    
                            let new_node_hash = new_node.hash();
                            
                            self.nodes.insert(new_node_hash, new_node);

                            if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                                current_node.children.insert(key[i].clone(), new_node_hash);
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
    pub fn remove(&mut self, key: Vec<K>) -> Result<(), Box<dyn Error>> {
        
        let mut current_node_hash = self.root;
        
        let mut i = 0;
        let last_i = key.len() - 1;

        while i < key.len() {

            if let Some(current_node) = self.nodes.get(&current_node_hash) {

                if current_node.key.is_empty() {

                    if i == last_i {

                        if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                            current_node.value = None;
                        }

                        self.rehash(&current_node_hash);

                        return Ok(());

                    } else {

                        match current_node.children.get(&key[i]) {
                            Some(next_node_hash) => {
                                current_node_hash = *next_node_hash;
                                i += 1;
                            },
                            None => return Ok(()),
                        }

                    }

                } else {

                    if &key[i..i + current_node.key.len()] == current_node.key.as_slice() {
                        
                        if i + current_node.key.len() == key.len() {
                            if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                                current_node.value = None;
                            }

                            self.rehash(&current_node_hash);

                            return Ok(());
                        
                        } else {
                            i += current_node.key.len();
                            current_node_hash = *current_node.children.get(&key[i]).ok_or("Child node not found!")?;
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

    // Search method
    pub fn search(&self, key: &[K]) -> Option<&V> {

        let mut current_node_hash = &self.root;
        let mut i = 0;

        while i < key.len() {
            
            if let Some(node) = self.nodes.get(current_node_hash) {

                if node.key.is_empty() {

                    if let Some(next_node_hash) = node.children.get(&key[i]) {
                        current_node_hash = next_node_hash;
                        i += 1;
                        continue;
                    } else {
                        return None;
                    }

                } else {
                    // Compressed key case
                    for (j, key_part) in node.key.iter().enumerate() {
                        if i + j >= key.len() || key_part != &key[i + j] {
                            // Mismatch or search_key is shorter than node_key
                            return None;
                        }
                    }

                    // Update search index after matching with compressed key
                    i += node.key.len();

                    // If it's a complete match of the search key
                    if i == key.len() {
                        return node.value.as_ref();
                    }

                    // Continue with children if there's more to search
                    if let Some(next_node_hash) = node.children.get(&key[i]) {
                        current_node_hash = next_node_hash;
                        i += 1; // Move to the next part of the search key
                        continue;
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
