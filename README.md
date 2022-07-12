# Fides

Fides is a cryptographic library for public key exchange, digital signatures, encryption, accumulation, hashing and zero knowledge proofs.

## Features

| Type | Technology | Algorithm |
|---|---|---|
| Public Key Exchange | Elliptic Curve | x25519 |
| | Lattices | Kyber-768 |
| Digital Signatures | Elliptic Curve | Ed25519 |
| | Lattices | Dilithium3 |
| Encryption | | chacha20poly1305 |
| Accumulators | Merkle Trees | Root |
| | Bloom Filters | Create, Insert, Search |
| Hashing | | Blake3 |
| Zero Knowledge | | Bulletproofs |
| Verifiable Delay Function | | |

## API

### Hashing

```text
let object: Vec<u8>;

let object_hash = hash(&object[..]);
```

### ChaCha20Poly1305

```text
let message: Vec<u8>;

let key: [u8; 32] = hash(&"password".as_bytes());

let cipher: Vec<u8> = chacha20poly1305::encrypt(&key, &message[..])?;

let plain: Vec<u8> = chacha20poly1305::decrypt(&key, &cipher[..])?;
```

### Ed25519

```text
let priv_key: [u8;32] = ed25519::private_key();

let signature: [u8; 64] = ed25519::sign(&message, &priv_key);

let pub_key: [u8;32] = ed25519::public_key(&priv_key);

let verification: bool = ed25519::verify(&message, &pub_key, &signature);
```

### x25519

```text
let priv_key: [u8;32] = x25519::private_key();

let pub_key: [u8;32] = x25519::public_key(&priv_key);

let shared_secret_key: [u8;32] = x25519::shared_key(&priv_key, &other_party_pub_key);
```

### Merkle Tree

```text
let hashes: Vec<[u8; 32]>;

let root = merkle_root(hashes);
```

2022-07-12
