# Fides

Fides is a library for cryptographic primitives.

## Author

Roy R. O. Okello

[Email](mailto:royokello@protonmail.com)

[Github](https://github.com/royokello)

[Twitter](https://twitter.com/RealOkello)

## Usage

### Cargo.toml

```text
[dependencies]
fides = "3.1.0"
```

### Module.rs

```text
use fides::{ ed25519, x25519, merkle_tree, hash::{ sha_2, sha_3 } };
```

## API

### ed25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`sign: message, secret_key -> signature`

`verify: message, public_key, signature -> bool`

### x25519

`secret_key -> secret_key`

`public_key: secret_key -> public_key`

`shared_key: public_key, secret_key -> shared_secret`

### merkle tree

`root: hasher, leaves -> hash`

### hash

`blake_3: input -> hash`

`sha_2::sha_224: input -> hash`

`sha_2::sha_256: input -> hash`

`sha_2::sha_512_224: input -> hash`

`sha_2::sha_512_256: input -> hash`

`sha_2::sha_384: input -> hash`

`sha_2::sha_512: input -> hash`

`sha_3::sha_224: input -> hash`

`sha_3::sha_256: input -> hash`

`sha_3::sha_384: input -> hash`

`sha_3::sha_512: input -> hash`

## Future

- Random Number Generator
- Hashing
- Encryption
- Public Key Exchange
- Digital Signatures
- Accumulation
- Commitment Schemes
- Zero-knowledge Proof Schemes
- Verifiable Delay Function
- Fully Homomorphic Encryption

## License

MIT
