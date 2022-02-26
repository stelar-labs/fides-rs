## Fides

Fides is a library for hashing with blake3, asymmetric cryptography on curve 25519 and symmetric cryptography with chacha20poly1305 written in Rust.

### Features
- Hashing.
- Symmetric Encryption.
- Symmetric Decryption.
- Asymmetric Private Key Generation.
- Asymmetric Public Key Generation.
- Asymmetric Shared Key Generation.
- Asymmetric Message Signing.
- Asymmetric Message Verification.

### Usage

In your `Cargo.toml`:

```

[dependencies]
fides = "1.0.0"

```

### API

`Hashing`
```

use fides::hash;

let blake3_hash: [u8;32] = hash(&bytes);

```

`Symmetric Encryption`
```

use fides::symmetric::encrypt;

let cipher: Vec<u8> = encrypt(&key, &msg);

```

`Symmetric Decryption`
```

use fides::symmetric::decrypt;

let plain: Vec<u8> = decrypt(&key, &cipher);

```

`Asymmetric Private Key Generation`
```

use fides::asymmetric::private_key;

let priv_key: [u8;32] = private_key();

```

`Asymmetric Public Key Generation`
```

use fides::asymmetric::public_key;

let pub_key: [u8;32] = public_key(&priv_key);

```

`Asymmetric Shared Key Generation`
```

use fides::asymmetric::shared_key;

let shared_secret_key: [u8;32] = shared_key(&priv_key, &other_party_pub_key);

```

`Asymmetric Message Signing`

```

use fides::asymmetric::sign;

let signature: [u8; 64] = sign(&message, &priv_key, &pub_key);

```

`Asymmetric Message Verification`

```

use fides::asymmetric::verify;

let verification: bool = verify(&message, &signer_public_key, &signature);

```

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-26
