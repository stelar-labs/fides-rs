use astro_format::IntoBytes;

use super::RadixTree;

impl<K, V> RadixTree<K, V>
where
    K: Eq + std::hash::Hash + Clone + std::cmp::Ord + IntoBytes + std::fmt::Display + std::fmt::Debug,
    V: Clone + IntoBytes + std::fmt::Debug,
{

    pub fn rehash(&mut self, child_hash: &[u8; 32]) {

        let (new_child_hash, parent_hash_opt) = if let Some(child_node) = self.nodes.remove(child_hash) {
            
            let new_child_hash = child_node.hash();

            self.nodes.insert(new_child_hash, child_node);

            if &self.root == child_hash {
                self.root = new_child_hash
            }

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
