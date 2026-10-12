# crowsi-device-credential-agent

Connect a managed device to the credential authority through the declared management contract.

## What you can do

- Validate device-management requests.
- Translate admitted requests into strict authority envelopes.

## Current scope

Device registration, trust and deployment-specific transport are prerequisites supplied by the operator.

Configuration follows the [config-v6 schema](schemas/config-v6.schema.json).
Root trust is explicitly supplied at `/etc/crowsi/device-credential-agent/root-trust.json`.
Authority transport is remote mTLS only over HTTPS; the operator supplies trust material
and private keys. See [provisioning](docs/provisioning.md) for the current trust boundary.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-device-credential-agent
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Detailed documentation](docs) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-credential](https://github.com/vpremises/vpremises-credential). Use the [product README](../../README.md) for composition, use cases and trust boundaries.
