# crowsi-credential-agent

Provide a dedicated local enrollment endpoint for adding credentials to an authorized custody service.

## What you can do

- Validate a bounded enrollment request.
- Connect enrollment to the configured local-control and broker interfaces.

## Current scope

This endpoint is enrollment-only. Its Unix socket and owner authorization must be provisioned explicitly.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-credential-agent
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/credential-custody](https://github.com/crowsi-net/credential-custody). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
