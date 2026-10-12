# crowsi-credential-authority-contracts

Exchange credential-authority requests without placing secret values in the contract.

## What you can do

- Represent device, grant and credential-reference relationships.
- Validate bounded proof and operation envelopes.

## Current scope

These types carry references and evidence. Custody and authorization remain with their configured services.

Package distribution is not activated by this documentation. Use the checked-in source and the declared dependency versions; published availability must be verified separately.

## Getting started

Install Rust 1.97 or newer and make the declared dependencies available. External dependencies come from crates.io. Run from the owning product workspace:

```sh
cargo test --locked
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)
