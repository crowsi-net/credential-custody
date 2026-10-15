# crowsi-owner-recovery-provider

Create and verify owner-recovery proofs through the fixed recovery profile.

## What you can do

- Use the released mnemonic and signing-key derivation rules.
- Verify recovery evidence through a narrow provider interface.

## Current scope

Recovery material is private registration data. The identity authority still decides whether a recovery operation is permitted.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-owner-recovery-provider
```

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/credential-custody](https://github.com/crowsi-net/credential-custody). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
