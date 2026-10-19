# Linux credential custody

## Dependency selection

The broker's optional `linux-secret-service` feature delegates storage, encryption
and unlock prompts to an existing OS Secret Service. Crowsi does not implement a
vault file format, cipher, password KDF, master-password cache or recovery cipher.
It does not enable a daemon, provision a collection or import real credentials.

The stable `libsecret` Rust crate is provided by GNOME's
[libsecret-rs project](https://gitlab.gnome.org/World/Rust/libsecret-rs). It is the
safe Rust binding to [GNOME libsecret](https://gitlab.gnome.org/GNOME/libsecret/),
not an independently assembled crypto implementation. The Rust binding is MIT;
the native libsecret library is LGPL-2.1-or-later. GLib/GIO and libsecret own the
local DBus connection, secret session negotiation, storage and unlock prompt.
This selection does not assert certification or an independent audit of the
exact selected release. Review ownership, advisories, native-library policy,
supported Rust versions, maintenance and licenses on update.

The initial oo7 candidate was rejected by RustSec audit on 2026-10-19: its DBus
stack still depended on three crates marked unmaintained on 2026-10-16
(RUSTSEC-2026-0370/0371/0372). No advisory exceptions or local upstream forks were
introduced. A famous direct dependency alone does not qualify its entire graph.

Build the optional adapter with the OS's `libsecret-1-dev` package (Debian/Ubuntu)
and existing GLib/GIO development dependencies. No native-library detection is
bypassed or replaced with invented SDK metadata. The Linux user authorizes any
local package installation; CI provisions its own native build dependencies.
No Crowsi file backend, custom cipher/KDF or plaintext-storage fallback is used.

## Use and trust boundary

Linux-only consumers select `default-features = false` and
`features = ["linux-secret-service"]`. The default `windows-custody` feature
preserves the Windows adapter, without forcing its dependencies into Linux-only
applications. Compatible zeroize releases are allowed rather than pinning a
shared dependency to an older patch release.

```rust,no_run
use crowsi_credential_broker::{LinuxCustodyStore, Result};
fn connect() -> Result<()> {
    // Provision a dedicated collection/alias using the OS provider beforehand.
    let store = LinuxCustodyStore::connect("crowsi")?;
    store.unlock()?; // The provider presents its own prompt, if needed.
    // Supply this store to Broker and the existing bound one-use authorization.
    store.lock()?;
    Ok(())
}
```

The feature is Linux-only and opt-in. The provider must be configured by the
owner. Require an encrypted persistent collection, never a passwordless one.
The protocol does not attest at-rest encryption: validate the actual OS provider
and policy. An unavailable unlock prompt is a failure, not permission to cache
a master password in Crowsi. Locking affects the entire selected collection;
select a dedicated collection to avoid locking unrelated applications.

Only opaque hashed slots and a generic label enter searchable attributes.
The validated envelope, revision and allowed-host checks stay inside the secret.
Use through Broker still requires bound authorization; the trusted storage
library alone is not an authorization service. This is software storage, not
hardware-nonexportable custody. Root or malicious same-user code is outside its
boundary. Removing a local entry does not revoke the remote credential.
Credentials are not automatically synchronized between computers.

## Failure and recovery

- Check collection/item locks before secret operations; never auto-unlock.
- Confirm writes by exact envelope/revision read-back and deletions by lookup.
- Refuse duplicate slots, wrong references, malformed envelopes and stale
  revisions. Metadata requires decoding a complete entry, not trusting attributes.
- Serialize calls per connection. Return fixed error codes without provider
  messages or secret values. Failure/timeout invalidates the connection; the
  adapter does not reconnect or automatically retry writes.
- A timeout or read-back failure does not prove that a write did not happen.
  Inspect via a new connection before recovery; do not blindly replay.
- Crash consistency, backups and password recovery belong to the provider.
  Crowsi does not promise stronger fsync/atomicity or rollback protection than it.
  Independent connections do not have a cross-process CAS guarantee.

## Verification and maintenance

`bash scripts/acceptance/linux-keyring.sh` uses isolated DBus and temporary XDG
directories with synthetic secrets and the actual GNOME Keyring provider.
It checks revision/slot boundaries, read-back, deletion, independent connections,
restart persistence, lock rejection and unavailable service. It does not touch
the user's keyring. The ignored Cargo provider target is explicitly executed by
this script and CI; a skip is not reported as acceptance. Actual desktop prompt,
real credential import and WordPress integration are separate acceptance steps.

Cargo.lock records the tested dependency closure. Dependabot checks Cargo and
Actions daily, with security-update PRs separate from routine version updates.
Repository settings must enable both alerts and automatic security-update PRs.
Feature-enabled CI, RustSec audit and actual provider acceptance precede merge.
RustSec also runs daily in this owner repository; that scheduled run does not run
the heavy workspace tests. It rejects maintenance warnings as well as known
vulnerabilities, including advisories that do not create a Dependabot alert.
No auto-merge or publication is configured. An alert without an available fix
remains for investigation rather than being suppressed.

The OS keyring daemon is not a Cargo dependency: Dependabot cannot update it.
The host's OS security-update channel must maintain it. Provider availability
and encryption policy are runtime prerequisites, not inferred from adding a crate.
