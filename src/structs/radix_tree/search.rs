use astro_format::IntoBytes;

use super::RadixTree;


impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Debug,
    V: Clone + IntoBytes + std::fmt::Debug,
{

    pub fn search<I>(&self, key: I) -> Option<&V>
        where
            I: IntoIterator<Item = K>,
        {

            let mut current_node_hash_opt = Some(&self.root);

            let mut key_parts = key.into_iter().peekable();

            while let Some(current_node_hash) = current_node_hash_opt {
                
                let current_key_part_opt = key_parts.next();

                if let Some(node) = self.nodes.get(current_node_hash) {

                    if node.key.is_empty() {

                        if key_parts.peek().is_none() {

                            return node.value.as_ref();

                        } else {

                            if let Some(current_key_part) = current_key_part_opt {

                                if let Some(next_node_hash) = node.children.get(&current_key_part) {
                                    current_node_hash_opt = Some(next_node_hash);
                                    continue;
                                } else {
                                    return None;
                                }
                            
                            } else {
                                return None;
                            }

                        }

                    } else {

                        let mut node_key_iter = node.key.iter().peekable();

                        if current_key_part_opt.as_ref() != node_key_iter.next() {
                            return None;
                        }

                        while let Some(node_key_part) = node_key_iter.next() {

                            if Some(node_key_part) != key_parts.peek() {
                                
                                return None
                            } else {
                                key_parts.next();
                            }
                        }

                        if let Some(next_key_part) = key_parts.peek() {

                            if let Some(next_node_hash) = node.children.get(&next_key_part) {
                                current_node_hash_opt = Some(next_node_hash);
                                key_parts.next();
                            } else {
                                return None;
                            }

                        } else {
                            return node.value.as_ref();
                        }

                    }
                } else {
                    return None;
                }

            }

            None
        }

}
