## Rust Fides

Fides is a library for hashing and symmetric/asymmetric cryptography written in Rust.

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
fides = "0.4.0"

```

### API

`Hashing`
```

use fides::hash;

let h: [u8;32] = hash(&bytes);

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

let sh_key: [u8;32] = shared_key(&priv_key, &pub_key);

```

`Asymmetric Message Signing`
`Asymmetric Message Verification`


### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-18
