# credential-custody

Reusable Crowsi components with independent trust roles. Product registration, CLI composition and installed native helpers belong to [vPremises environment operations](https://github.com/vpremises/environment-operations).

## Packages

- [crowsi-credential-agent](crates/crowsi-credential-agent): existing crate identity and publication declaration retained.
- [crowsi-credential-authority](crates/crowsi-credential-authority): existing crate identity and publication declaration retained.
- [crowsi-credential-authority-contracts](crates/crowsi-credential-authority-contracts): existing crate identity and publication declaration retained.
- [crowsi-credential-broker](crates/crowsi-credential-broker): existing crate identity and publication declaration retained.
- [crowsi-credential-runtime](crates/crowsi-credential-runtime): existing crate identity and publication declaration retained.
- [crowsi-device-credential-agent](crates/crowsi-device-credential-agent): existing crate identity and publication declaration retained.
- [crowsi-owner-recovery-provider](crates/crowsi-owner-recovery-provider): existing crate identity and publication declaration retained.
- [crowsi-pa-key-agent](crates/crowsi-pa-key-agent): existing crate identity and publication declaration retained.
- [crowsi-windows-custody-provider](crates/crowsi-windows-custody-provider): existing crate identity and publication declaration retained.
- [crowsi-windows-operation-contracts](crates/crowsi-windows-operation-contracts): existing crate identity and publication declaration retained.

## Verify and explore

```sh
cargo +1.97 test --locked --workspace --all-targets --all-features
cargo +1.97 test --locked --workspace --doc --all-features
```

Each crate README and examples describe its public API. Tests and examples use synthetic state; they do not install an authority, service or maintenance timer. Authority, broker, enforcement, independent verification and recovery retain separate roles. Existing wire/schema identifiers and publish=false declarations are unchanged. Repository relocation does not imply a new registry release. Original public history and license notices are preserved in docs/source-history.

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.
