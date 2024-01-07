use std::error::Error;

use astro_format::IntoBytes;

use super::{RadixTree, RadixNode};

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Display,
    V: Clone + IntoBytes,
{

    pub fn remove<I>(&mut self, key: I) -> Result<(), Box<dyn Error>> where I: IntoIterator<Item = K>, {
            
        let mut node_hash = self.root;
        
        let mut key_iter = key.into_iter().peekable();


        while let Some(k) = key_iter.next() {

            match self.nodes.get(&node_hash) {

                Some(node) => {

                    if node.key.is_empty() {

                        if key_iter.peek().is_none() {

                            if let Some(mut_node) = self.nodes.get_mut(&node_hash) {
                                mut_node.value = None;
                                self.rehash(&node_hash);
                                return Ok(());
                            }

                        } else {

                            match node.children.get(&k) {
                                Some(next_node_hash) => {
                                    node_hash = *next_node_hash;
                                },
                                None => return Ok(()),
                            }

                        }

                    } else {

                        let mut key_matched = true;
    
                        let mut node_key_iter = node.key.iter().peekable();
    
                        if Some(&k) != node_key_iter.next() {
                            key_matched = false;
                        }
    
                        while let Some(nk) = node_key_iter.next() {
                            if Some(nk) != key_iter.peek() {
                                key_matched = false;
                                break;
                            } else {
                                key_iter.next();
                            }
                        }

                        if key_matched && key_iter.peek().is_none() {

                            if let Some(mut_node) = self.nodes.get_mut(&node_hash) {

                                if mut_node.children.is_empty() {

                                    if let Some(parent_hash) = self.parents.remove(&node_hash) {

                                        let parent_node_opt = match self.nodes.get(&parent_hash) {
                                            Some(parent_node) => Some(parent_node.clone()),
                                            None => None,
                                        };

                                        if let Some(mut parent_node) = parent_node_opt {
                                                
                                            let remove_key = parent_node.children.iter().find_map(|(k, &v)| {
                                                if v == node_hash {
                                                    Some(k.clone())
                                                } else {
                                                    None
                                                }
                                            });

                                            if let Some(rk) = &remove_key {
                                                parent_node.children.remove(rk);
                                            }

                                            if parent_node.children.len() == 0 {

                                                self.nodes.remove(&parent_hash);
    
                                                if self.root == parent_hash {
                                                    self.root = [0;32];
                                                    break;
                                                }
    
                                            } else if parent_node.children.len() == 1 && parent_node.value.is_none() {

                                                self.nodes.remove(&node_hash);

                                                if let Some((child_key, child_hash)) = parent_node.children.first_key_value() {

                                                    if let Some(child_node) = self.nodes.get(child_hash) {

                                                        let mut new_key = parent_node.key.clone();
                                                        new_key.push(child_key.clone());
                                                        new_key.extend(child_node.key.clone());

                                                        let join_node = RadixNode {
                                                            children: child_node.children.clone(),
                                                            key: new_key,
                                                            value: child_node.value.clone(),
                                                        };

                                                        self.nodes.insert(parent_hash, join_node);

                                                        self.rehash(&parent_hash);

                                                    }

                                                    self.parents.remove(child_hash);

                                                    self.nodes.remove(child_hash);
                                                    
                                                };

                                                return Ok(());
    
                                            } else {
                                                break;
                                            }

                                            self.nodes.insert(parent_hash, parent_node.clone());

                                            self.rehash(&parent_hash);

                                        } else {
                                            return Ok(())
                                        }

                                    }
                                  
                                    self.nodes.remove(&node_hash);

                                } else {

                                }
                            }

                        } else if key_matched {

                            if let Some(k_next) = key_iter.next() {
                                
                                if let Some(next_node_hash) = node.children.get(&k_next) {
                                    node_hash = *next_node_hash;
                                } else {
                                    return Err("Invalid Child!")?;
                                }
                            }

                        } else {
                            return Ok(())
                        }

                    }

                },
                None => Err("Invalid node!")?,
            }

        }

        Ok(())
    
    }

}
