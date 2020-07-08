# E114 Access provides utils to enforce an API access policies
This crate is:
```rust
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
```

# ☠☡☣☢ This crate is in rapid flux, do not rely on it just yet! ☢☣☡☠
You have been warned.

## Minimum rust version
1.40

## Optional features
```toml
e114_access = { version = "0.1", features = ["jsonschema"] }
```
Adds `#[derive(JsonSchema)]` to certain types and the `schemars` dependency.

```toml
e114_access = { version = "0.1", features = ["validate"] }
```
Enables <keats/validator> and `contracts` checks.

## Generate new PEMs
These are used for token signing and validation.
The api only needs the public key, the private key is only used by the auth service during token generation.

```bash
openssl genrsa -out web_id_rsa.pem 4096
```
```bash
openssl rsa -in web_id_rsa.pem -inform PEM -RSAPublicKey_out -outform PEM -out web_id_rsa_pub.pem
```

## Build, debug and release tools
- cargo fmt & cargo +1.40.0 clippy --tests
