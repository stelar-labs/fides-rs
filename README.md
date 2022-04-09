# Fides

Fides is a library for hashing with blake3, asymmetric cryptography on curve 25519, symmetric cryptography with chacha20poly1305 and merkle tree functions.

## Usage

In your `Cargo.toml`:

```

[dependencies]
fides = "2.2.1"

```

In your `module.rs`:

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
let message: Vec<u8>;

let key: [u8; 32] = hash(&"password".as_bytes());

let cipher: Vec<u8> = chacha20poly1305::encrypt(&key, &message).unwrap();

let plain: Vec<u8> = chacha20poly1305::decrypt(&key, &cipher).unwrap();
```

### Ed25519

```
let priv_key: [u8;32] = ed25519::private_key();

let signature: [u8; 64] = ed25519::sign(&message, &priv_key);

let pub_key: [u8;32] = ed25519::public_key(&priv_key);

let verification: bool = ed25519::verify(&message, &pub_key, &signature);
```

### x25519

```
let priv_key: [u8;32] = x25519::private_key();

let pub_key: [u8;32] = x25519::public_key(&priv_key);

let shared_secret_key: [u8;32] = x25519::shared_key(&priv_key, &other_party_pub_key);
```

### Merkle Tree

```
let objects: Vec<Vec<u8>>;

let root: [u8; 32] = merkle_root(&objects);
```

## Contribution

Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-09
