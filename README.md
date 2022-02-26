## Fides

Fides is a wrapper for hashing with blake3, asymmetric cryptography on curve 25519 and symmetric cryptography with chacha20poly1305 written in Rust.

### Features
- Blake3 Hashing.
- ChaCha20Poly1305 Encryption.
- ChaCha20Poly1305 Decryption.
- Ed25519 Private Key Generation.
- Ed25519 Public Key Generation.
- Ed25519 Message Signing.
- Ed25519 Message Verification.
- x25519 Private Key Generation.
- x25519 Public Key Generation.
- x25519 Shared Key Generation.

### Usage

In your `Cargo.toml`:

```

[dependencies]
fides = "2.0.0"

```

### API

`Hashing`
```

use fides::hash;

let blake3_hash: [u8;32] = hash(&bytes);

```

`Symmetric Encryption`
```

use fides::chacha20poly1305::encrypt;

let cipher: Vec<u8> = encrypt(&key, &msg);

```

`Symmetric Decryption`
```

use fides::chacha20poly1305::decrypt;

let plain: Vec<u8> = decrypt(&key, &cipher);

```

`Asymmetric Private Key Generation`
```

use fides::Ed25519::private_key;

let priv_key: [u8;32] = private_key();

```

`Asymmetric Public Key Generation`
```

use fides::Ed25519::public_key;

let pub_key: [u8;32] = public_key(&priv_key);

```

`Asymmetric Shared Key Generation`
```

use fides::x25519::shared_key;

let shared_secret_key: [u8;32] = shared_key(&priv_key, &other_party_pub_key);

```

`Asymmetric Message Signing`

```

use fides::Ed25519::sign;

let signature: [u8; 64] = sign(&message, &priv_key, &pub_key);

```

`Asymmetric Message Verification`

```

use fides::Ed25519::verify;

let verification: bool = verify(&message, &signer_public_key, &signature);

```

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-26
