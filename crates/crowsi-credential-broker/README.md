# crowsi-credential-broker

Keep credentials in custody and lend their use through a narrowly scoped, one-use lease.

## What you can do

- Register and resolve opaque credential references.
- Limit credential use to an authorized purpose.

## Current scope

Applications must configure custody and the authorization boundary. Secret values must not be copied into configuration or logs.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-credential-broker
```

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Examples](examples) · [Schemas](schemas) · [Detailed documentation](docs) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/credential-custody](https://github.com/crowsi-net/credential-custody). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
