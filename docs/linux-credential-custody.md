# Linux credential custody

## Dependency selection

The broker's optional `linux-secret-service` feature delegates storage, encryption
and unlock prompts to an existing OS Secret Service. Crowsi does not implement a
vault file format, cipher, password KDF, master-password cache or recovery cipher.
It does not enable a daemon, provision a collection or import real credentials.

The stable `oo7` Rust crate is maintained by linux-credentials and used in GNOME's
credential infrastructure. GNOME's [project report](https://blogs.gnome.org/tbernard/2025/04/11/gnome-stf-2024/)
describes this integration; [upstream source](https://github.com/linux-credentials/oo7)
and [API documentation](https://docs.rs/oo7/0.6.0/oo7/) identify its MIT license and
supported backends. This is a platform security component, not a claim that this
exact release has an independent audit, certification or vendor support SLA.
Re-evaluate ownership, advisories, Rust support, maintenance and license on update.

Use an explicit encrypted DBus session (`Service::encrypted`) and existing
collection alias. Automatic-backend selection, file storage, unstable API,
plain sessions and tracing are not used. `native_crypto` is upstream protocol
implementation; Crowsi does not assemble its primitives. No fallback creates a
plaintext store when DBus, encryption or the provider fails.

## Use and trust boundary

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
No auto-merge or publication is configured. An alert without an available fix
remains for investigation rather than being suppressed.

The OS keyring daemon is not a Cargo dependency: Dependabot cannot update it.
The host's OS security-update channel must maintain it. Provider availability
and encryption policy are runtime prerequisites, not inferred from adding a crate.
