use std::collections::BTreeMap;

use astro_format::IntoBytes;

use super::{RadixTree, RadixNode};


impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes,
    V: Clone + IntoBytes,
{

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

                let mut key_iter = key.into_iter().peekable();

                while let Some(k) = key_iter.next() {

                    let current_node_key_opt = match self.nodes.get(&current_node_hash) {
                        Some(current_node) => Some(current_node.key.clone()),
                        None => None,
                    };

                    if let Some(current_node_key) = current_node_key_opt {

                        if key_iter.peek().is_none() {

                            if let Some(current_node) = self.nodes.get_mut(&current_node_hash) {
                                current_node.value = Some(value);
                                self.rehash(&current_node_hash);
                            }

                            return;

                        } else {

                            let mut split_position = 0;
                            let mut matched = true;

                            let mut current_node_key_iter = current_node_key.iter();

                            if Some(&k) != current_node_key_iter.next() {
                                matched = false;
                            } else {
                                split_position += 1;
                            }

                            while let Some(ck) = current_node_key_iter.next() {

                                if let Some(next_key_part) = key_iter.peek() {

                                    if next_key_part == ck {

                                        key_iter.next();

                                    } else {
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

                                        let child_key = key_iter.next().unwrap();

                                        let mut new_node_key = vec![];
                                
                                        if key_iter.peek().is_some() {
                                            while let Some(key_part) = key_iter.next() {
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

                                        if let Some(left_node) = self.nodes.get_mut(&left_node_hash) {
                                            left_node.children.insert(child_key, new_node_hash);
                                        }
                                        
                                        self.parents.insert(new_node_hash, current_node_hash);

                                        self.rehash(&current_node_hash);

                                    },
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        return;
                                    }
                                }

                            } else {

                                let mut new_node_key = vec![];
                                
                                if key_iter.peek().is_some() {
                                    while let Some(key_part) = key_iter.next() {
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
                                    current_node.children.insert(k, new_node_hash);
                                }

                                self.parents.insert(new_node_hash, current_node_hash);

                                self.rehash(&current_node_hash);

                            }

                        }

                    }

                }
            
            }

        }

}
