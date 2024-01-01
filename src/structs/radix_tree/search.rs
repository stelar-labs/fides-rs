use astro_format::IntoBytes;

use super::RadixTree;


impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Display + std::fmt::Debug,
    V: Clone + IntoBytes + std::fmt::Debug,
{

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

                            return node.value.as_ref();
                        
                        } else if key_matched {

                            if let Some(next_key_part) = key_parts.next() {

                                if let Some(next_node_hash) = node.children.get(&next_key_part) {
                                    current_node_hash = next_node_hash;
                                } else {
                                    return None;
                                }

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

}
