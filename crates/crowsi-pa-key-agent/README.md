# crowsi-pa-key-agent

Manage policy-administrator signing keys and expose only their approved public state.

## What you can do

- Initialize explicitly selected key custody.
- Perform bounded signing through the declared key interface.

## Current scope

The operator chooses private state and custody configuration. Private key material must not enter public projections.

The built-in credential-authorization policy permits credential enrollment only.
Provider-observation requests are rejected before creating a challenge or signing
an envelope. The retired application-specific GitHub observer is not a generic
provider contract. Existing credentials, external App registrations, trust stores
and previously issued grants are not revoked by this source change.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-pa-key-agent
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/credential-custody](https://github.com/crowsi-net/credential-custody). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
