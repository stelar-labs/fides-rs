## Rust Fides

Fides is a library for hashing and symmetric/asymmetric cryptography written in Rust.

### Features
- Hashing.
- Symmetric Encrypt.
- Symmetric Decrypt.
- Asymmetric Public Key Generation.
- Asymmetric Shared Key Generation.
- Asymmetric Message Signing.
- Asymmetric Message Verification.

### Usage

In your `Cargo.toml`:

```

[dependencies]
fides = "0.3.0"

```

### API
`Hashing`
```

use fides::hash;

let h: Vec<u8> = hash(&bytes);

```


### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.
