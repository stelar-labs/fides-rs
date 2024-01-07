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
- `key`: Iterable of type `K`.
- `value`: Value to be associated with the key.

`rehash(&mut self, child_hash: &[u8; 32])`: Updates the hash of a child node.
- `child_hash`: Hash of the child node.

`remove<I>(&mut self, key: I) -> Result<(), Box<dyn Error>>`: Removes a key-value pair.
- `key`: Iterable of type `K`.

`search<I>(&self, key: I) -> Option<&V>`: Searches for a value by key.
- `key`: Iterable of type `K`.

`split_node(&mut self, node_hash: [u8; 32], split_position: usize) -> Result<([u8; 32], [u8; 32]), Box<dyn Error>>`: Splits a node at a specified position.
- `node_hash`: Hash of the node to split.
- `split_position`: Position to split the node.

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
