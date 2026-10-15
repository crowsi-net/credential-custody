# crowsi-windows-custody-provider

Protect owner-local credential storage using Windows CurrentUser DPAPI and a verified native helper.

## What you can do

- Access protected custody from the declared Rust client.
- Pin the helper identity before a bounded operation.

## Current scope

Windows user context and the registered helper are required, including when the caller runs in WSL. This is not a Linux Secret Service backend.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-windows-custody-provider
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/credential-custody](https://github.com/crowsi-net/credential-custody). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
