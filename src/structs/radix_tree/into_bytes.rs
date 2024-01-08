use astro_format::IntoBytes;

use super::{RadixNode, RadixTree};

impl<K, V> IntoBytes for RadixNode<K, V>
where
    K: IntoBytes + Clone + Eq,
    V: IntoBytes + Clone,
{
    fn into_bytes(&self) -> Vec<u8> {

        let children_bytes = astro_format::encode(
            self.children.iter().map(|(k,v)| {
                astro_format::encode([k.into_bytes(), v.to_vec()]).unwrap_or_default()
            })).unwrap_or_default();

        let key_bytes = astro_format::encode(self.key.iter().map(|x| x.into_bytes())).unwrap_or_default();

        let value_bytes = match &self.value {
            Some(val) => astro_format::encode([vec![1], val.into_bytes()]).unwrap_or_default(),
            None => astro_format::encode([vec![0]]).unwrap_or_default(),
        };

        astro_format::encode([children_bytes, key_bytes, value_bytes]).unwrap_or_default()

    }
}

impl<K, V> IntoBytes for RadixTree<K, V>
where
    K: IntoBytes + Clone + Eq,
    V: IntoBytes + Clone,
{
    fn into_bytes(&self) -> Vec<u8> {

        let root_bytes = self.root.to_vec();

        let nodes_bytes = astro_format::encode(
            self.nodes.iter().map(|(_, node)| { node.into_bytes() })
        ).unwrap_or_default();

        astro_format::encode([root_bytes, nodes_bytes]).unwrap_or_default()

    }
}
