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

Each crate README and examples describe its public API. Tests and examples use synthetic state; they do not install an authority, service or maintenance timer. Authority, broker, enforcement, independent verification and recovery retain separate roles. Existing wire/schema identifiers and publish=false declarations are unchanged. Repository relocation does not imply a new registry release. Original repository/revision provenance is retained in source-consolidation.json; current LICENSE/NOTICE files remain with their owned source.

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.

[Local execution and remote review](docs/local-acceptance.md) defines the required
source-bound local namespace acceptance before push and normal PR merge.

Dependency updates: [repository policy](docs/dependency-updates.md).
Linux credential storage: [OS provider delegation and fault handling](docs/linux-credential-custody.md).
Registry releases: [version tags and GitHub OIDC](docs/registry-release.md).
