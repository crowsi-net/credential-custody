# crowsi-windows-operation-contracts

Validate the narrow requests exchanged with a Windows helper process.

## What you can do

- Pin helper identity and operation limits.
- Represent permitted helper requests and results.

## Current scope

These contracts do not install a helper or implement a Windows custody backend.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-windows-operation-contracts
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-credential](https://github.com/vpremises/vpremises-credential). Use the [product README](../../README.md) for composition, use cases and trust boundaries.
