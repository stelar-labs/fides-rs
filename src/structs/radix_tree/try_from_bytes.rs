use astro_format::{TryFromBytes, IntoBytes};
use std::collections::{BTreeMap, HashMap, HashSet};
use super::{RadixNode, RadixTree};

impl<'a, K, V> TryFromBytes<'a> for RadixNode<K, V>
where
    K: TryFromBytes<'a> + Clone + Eq + Ord,
    V: TryFromBytes<'a> + Clone,
{
    fn try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>> {
        
        let decoded: Vec<&'a [u8]> = astro_format::decode(&value)?;

        let mut children = BTreeMap::new();

        let children_bytes = decoded.get(0).ok_or("missing children bytes!")?;

        if !children_bytes.is_empty() {

            let children_tuples: Vec<&'a [u8]> = astro_format::decode(children_bytes)?;

            for child_tuple in children_tuples {

                let child_decoded: Vec<&[u8]> = astro_format::decode(&child_tuple)?;

                if child_decoded.len() != 2 {
                    return Err("Child tuple must have 2 elements".into());
                }

                let key = K::try_from_bytes(child_decoded[0])?;
                let value: [u8; 32] = child_decoded[1].try_into().map_err(|_| "Invalid hash length")?;

                children.insert(key, value);
            }

        }
        
        let key_bytes = decoded.get(1).ok_or("missing key bytes!")?;

        let key = if key_bytes.is_empty() {
            vec![]
        } else {
            astro_format::decode(key_bytes)?
                .into_iter()
                .map(K::try_from_bytes)
                .collect::<Result<Vec<K>, _>>()?
        };

        
        let value_bytes = decoded.get(2).ok_or("missing value bytes!")?;
        
        let value = if !value_bytes.is_empty() {

            let value_decoding: Vec<&[u8]> = astro_format::decode(value_bytes)?;

            let value_type = value_decoding.get(0).ok_or("missing value type!")?;

            match value_type {
                [0] => None,
                [1] => V::try_from_bytes(value_decoding.get(1).ok_or("missing value payload!")?).ok(),
                _ => Err("invalid value type!")?
            }

        } else {
            Err("missing value data!")?
        };

        Ok(RadixNode {
            children,
            key,
            value,
        })
    }
}

impl<'a, K, V> TryFromBytes<'a> for RadixTree<K, V>
where
    K: TryFromBytes<'a> + Clone + Eq + Ord + IntoBytes,
    V: TryFromBytes<'a> + Clone + IntoBytes,
{
    fn try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>> {

        let decoded: Vec<&'a [u8]> = astro_format::decode(value)?;

        let mut nodes = HashMap::new();
        let mut parents = HashMap::new();
        let mut potential_roots = HashSet::new();

        for d in decoded {
            
            let node: RadixNode<K, V> = RadixNode::try_from_bytes(d)?;
            let hash = node.hash(); // Assuming RadixNode has a hash method
            nodes.insert(hash, node.clone());

            // Initially consider every node as a potential root
            potential_roots.insert(hash);

            for (_, child_hash) in node.children.iter() {
                parents.insert(*child_hash, hash);
                // Remove child nodes from potential roots
                potential_roots.remove(child_hash);
            }
        }

        // The root is the only node not present as a child in the `parents` HashMap
        let root = potential_roots.into_iter().next().ok_or("Root node not found")?;

        Ok(RadixTree { nodes, parents, root })

    }

}