use std::error::Error;

use astro_format::IntoBytes;

use super::RadixTree;

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Display + std::fmt::Debug,
    V: Clone + IntoBytes + std::fmt::Debug,
{

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

                    if Some(&current_key_part) != key_iter.next() {
                        break;
                    }

                    while let Some(ck) = key_iter.next() {
                        if Some(ck) != key_parts.next().as_ref() {
                            key_matched = false;
                            break;
                        }
                    }

                    if key_matched && key_iter.next().is_none() && key_parts.peek().is_none() {
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

}
