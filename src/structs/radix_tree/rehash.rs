use astro_format::IntoBytes;

use super::RadixTree;

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes,
    V: Clone + IntoBytes,
{

    pub fn rehash(&mut self, child_hash: &[u8; 32]) {

        if let Some(child_node) = self.nodes.remove(child_hash) {

            let new_child_hash = child_node.hash();

            for (_, key_hash) in &child_node.children {
                self.parents.insert(*key_hash, new_child_hash);
            }

            self.nodes.insert(new_child_hash, child_node);

            if &self.root == child_hash {
                self.root = new_child_hash
            }

            if let Some(parent_hash) = self.parents.remove(child_hash) {

                if let Some(parent_node) = self.nodes.get_mut(&parent_hash) {

                    if let Some((child_key, _)) = parent_node.children.iter().find(|(_,v)| *v == child_hash) {
                        parent_node.children.insert(child_key.clone(), new_child_hash);
                    }

                    self.rehash(&parent_hash);

                }

            }

        }

    }

}
