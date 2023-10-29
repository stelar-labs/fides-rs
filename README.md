# Fides

Fides is a library for cryptographic primitives.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Digital Signature Algorithms
- Hashing
- Public Key Exchange
- Data Structures

## Usage

### Installation

- From Crates by adding `fides = "3.2.0"` to `Cargo.toml` under `[dependencies]`

### Digital Signature Algorithms

#### ed25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`sign: message, secret_key -> signature`

`verify: message, public_key, signature -> bool`

### Public Key Exchange

#### x25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`shared_key: public_key, secret_key -> shared_secret`

### Data Structures

#### Bloom Filter

#### merkle tree

`root: hasher, leaves -> hash`

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
