# Authority process acceptance

This is a partial development acceptance of authority process boundaries, not
the six-scenario endpoint/provider product acceptance. No credential is installed,
no host `/etc` path is modified, and no real identity or provider is contacted.

Run the existing owner target:

```sh
cargo test --locked -p crowsi-credential-authority \
  --features test-support,authority-host-test-root --test gateway_host_subprocess
```

The four additional cases execute the real finite authority host binary in
separate, bounded child processes. Each has private temporary directories,
synthetic signing keys and a real anchored authority store. Configuration
digests, identity signatures and store checks run in production host code.
The existing debug-only `authority-host-test-root` feature is forbidden in
release builds; it locates fixture trust beside the signed configuration.

| Case | Evidence | Limit |
| --- | --- | --- |
| A epoch advances, B persists | Mutate A using a synthetic verified B step-up, then a fresh host child reads epoch 8 for A and an identical B entry | Does not execute a user-verification ceremony, terminate A sessions or exercise grants |
| A identity copied to B | Real host refuses A's signed identity with B's registered peer; a legitimate A request still succeeds | Does not copy an OS vault/passkey or test a TLS handshake |
| Identity response tampered | Typed signed exchange is changed; host refuses, both device epochs remain 7 | Does not test audit-log tampering or authority outage |
| Existing state absent | Read request refuses instead of initializing/registering an authority | Does not implement or accept legacy re-enrollment |

The existing two cases also test a signed snapshot and a signed peer head.
Six tests in this target are **not** the six end-to-end product journeys.

Remaining product acceptance needs actual endpoint-agent and identity/provider
processes, transport authentication, revocation/session/grant use, copied
endpoint state, stale evidence, provider scope refusal and independent recovery.
Its in-process fake transports are useful unit tests, not a substitute for these
journeys. No fixture is named `production_multidevice.rs` merely to satisfy an
old file-presence check. The complete product acceptance remains unfinished
until the real journeys pass.

The repository's existing all-features CI runs this registered integration
target. Development results, source merge and any artifact publication are
separate acceptance states.

## Ordinary agent bootstrap in an isolated installation

`crowsi-device-credential-agent` registers `production_bootstrap` as a mandatory
integration target. It runs the ordinary CLI binary, with no alternate trust
path, injected core or production authentication branch. A permanent test harness
creates fresh Linux user, mount, PID and network namespaces and a private root
filesystem. Synthetic installation trust is at the real `/etc/crowsi` path **in
that namespace**. The host trust file's metadata is compared before and after.
The existing transport/core test constructors only supply test document shapes;
they do not execute the agent in place of its real process.

Four cases cover valid explicit initialization, idempotent preservation of existing state,
invalid/writable installed trust, and changed configuration without a signature.
This closes the bootstrap testability gap without adding a test trust feature to
the product. Missing namespace support, mount tools or privileges is a failed
prerequisite, never a skipped/passed product acceptance. No Docker, host mount,
sysctl, package installation or persistent permission change is performed.

Both target suites test lower boundaries. They do not yet test the complete
endpoint → authenticated transport → authority → provider/recovery journeys.
