# Fides

Fides is a library for hashing with blake3, asymmetric cryptography on curve 25519, symmetric cryptography with chacha20poly1305 and merkle tree functions written in Rust.

## Usage

In your `Cargo.toml`:

```

[dependencies]
fides = "2.0.0"

```

In your module:

```

use fides::{chacha20poly1305, ed25519, hash, merkle_root, x25519};

```

## API

### Hashing

```
let bytes: Vec<u8>;

let blake3_hash: [u8;32] = hash(&bytes);
```

### ChaCha20Poly1305

```
let key: [u8; 32] = hash(&"password".as_bytes());

let cipher: Vec<u8> = chacha20poly1305::encrypt(&key, &msg);

let plain: Vec<u8> = decrypt(&key, &cipher);
```

### Ed25519

```
let priv_key: [u8;32] = ed25519::private_key();

let pub_key: [u8;32] = ed25519::public_key(&priv_key);

let signature: [u8; 64] = sign(&message, &priv_key, &pub_key);
```

### x25519

```
let priv_key: [u8;32] = x25519::private_key();

let pub_key: [u8;32] = x25519::public_key(&priv_key);

let shared_secret_key: [u8;32] = x25519::shared_key(&priv_key, &other_party_pub_key);

let verification: bool = x25519::verify(&message, &signer_public_key, &signature);
```

### Merkle Tree

```
let objects: Vec<Vec<u8>>;

let root: [u8; 32] = merkle_root(&objects);
```

## Contribution

Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-08
