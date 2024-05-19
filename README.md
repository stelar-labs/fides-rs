# Fides

Fides is a library for cryptographic primitives.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Digital Signature Algorithms
- Hashing
- Public Key Exchange
- Data Structures like Bloom Filters & Radix Tree

## Digital Signature Algorithms

### ed25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`sign: message, secret_key -> signature`

`verify: message, public_key, signature -> bool`

## Public Key Exchange

### x25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`shared_key: public_key, secret_key -> shared_secret`

## Data Structures

### Bloom Filter

### Radix Tree

**Structs**

`RadixTree<K, V>`: Represents the radix tree, generic over `K` for key and `V` for value.
- `nodes`: `HashMap` storing the nodes of the tree, each identified by a 32-byte array.
- `parents`: `HashMap` tracking the parent of each node.
- `root`: 32-byte array representing the root node.

`RadixNode<K, V>`: Represents a single node within the `RadixTree`.
- `children`: `BTreeMap` mapping keys to child node hashes.
- `key`: `Vec<K>` representing the node's key.
- `value`: Optional `V` holding the value associated with the key.

**Methods**

`new() -> Self`: Creates a new instance of `RadixTree`.

`insert<I>(&mut self, key: I, value: V)`: Inserts a key-value pair into the tree.

`rehash(&mut self, child_hash: &[u8; 32])`: Updates the hash of a child node.

`remove<I>(&mut self, key: I) -> Result<(), Box<dyn Error>>`: Removes a key-value pair.

`search<I>(&self, key: I) -> Option<&V>`: Searches for a value by key.

`split_node(&mut self, node_hash: [u8; 32], split_position: usize) -> Result<([u8; 32], [u8; 32]), Box<dyn Error>>`: Splits a node at a specified position.

**Traits**

IntoBytes: Provides functionality to serialize RadixTree and RadixNode into bytes.

- For RadixNode<K, V>:

`into_bytes(&self) -> Vec<u8>`: Serializes the RadixNode into a byte vector. This includes serializing the children, key, and value fields.

- For RadixTree<K, V>:

`into_bytes(&self) -> Vec<u8>`: Serializes the entire RadixTree into a byte vector. This process involves serializing the root and each node in nodes (using RadixNode's into_bytes method).


### Merkle Tree

**Structs**

`MerkleTree<T>`: Represents the Merkle Tree, generic over T for the data stored in each node.
- `nodes`: `HashMap<[u8; 32], MerkleNode<T>>` storing the nodes of the tree, each identified by a 32-byte array.
- `root`: `[u8; 32]` representing the root node.
- `parents`: `HashMap<[u8; 32], [u8; 32]>` tracking the parent of each node.

**Methods**

`append(mut self, data: T)`: Appends data to the Merkle Tree.
`replace(mut self, index: usize, data: T)`: Replaces the data at the specified index in the Merkle Tree.

**Traits**

IntoBytes: Provides functionality to serialize MerkleTree and MerkleNode into bytes.

- For MerkleTree<T>: `into_bytes(&self) -> Vec<u8>`
- For MerkleNode<T>: `into_bytes(&self) -> Vec<u8>`

TryFromBytes: Provides functionality to deserialize MerkleTree and MerkleNode from bytes.

- For MerkleTree<T>: `try_from_bytes(value: &[u8]) -> Result<Self, Box<dyn Error>>`
- For MerkleNode<T>: `try_from_bytes(value: &[u8]) -> Result<Self, Box<dyn Error>>`

## Future

- 🎲 Random Number Generator
- 🔒 Encryption
- 🏗 Accumulation
- 📜 Commitment Schemes
- 😎 Zero-knowledge Proof Schemes
- ⏲ Verifiable Delay Function
- 🔢 Fully Homomorphic Encryption

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
