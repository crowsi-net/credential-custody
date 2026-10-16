# Local execution and remote source review

The owner selects local Linux execution followed by remote push for tests needing
user/mount/PID/network namespaces. Standard GitHub-hosted CI runs portable tests,
formatting, Clippy, doctests and receipt consistency; it does not claim to have
executed the local namespace target. No self-hosted runner registration is needed.

## Rechecked failure and security scope

Run 38878796955 failed in `unshare` while writing `/proc/self/uid_map`, before
the namespace helper or ordinary agent CLI executed. All four bootstrap cases
failed at setup, not at credential validation. Ubuntu 24.04 documents restricted
unprivileged user namespaces, including AppArmor capability restrictions:
https://documentation.ubuntu.com/security/security-features/privilege-restriction/apparmor/
The old log alone cannot identify the exact LSM/seccomp policy that denied the
mapping. The read-only diagnostic records available policy knobs and the minimal
probe on subsequent CI runs. Do not claim an AppArmor diagnosis without evidence.

Review found the helper verified user/mount isolation but relied on flags alone
for PID/network separation and recursively bound host runtime trees. It now checks
all four namespace identities and applies recursive read-only, nosuid/nodev
attributes, then verifies every resulting runtime mount. Locked nested mounts
can reject non-recursive binds in a user namespace; importing them without
recursive restrictions is not accepted. Mount propagation is private.
The child uses a clean environment, bounded lifetime, isolated proc and temporary
root; ordinary installed trust checks and signature validation remain unchanged.
Synthetic signing keys are test-only and never installed on the host. No alternate
trust feature, production fallback, host /etc write, privilege grant, sudo, sysctl
or AppArmor relaxation is introduced. These tests are not a full kernel sandbox
proof or the six multidevice journeys required by coela-release #118.

## Operator flow

Use the existing non-root Linux toolchain (Rust 1.97 and Node supporting .mts type
stripping). Inspect the source diff first; never run untrusted PR code on a machine
holding real credentials. Use a clean checkout and synthetic test state only.

```sh
git status --short
sh scripts/acceptance/diagnose.sh
node --experimental-strip-types scripts/acceptance/local.mts run
git add acceptance/evidence/
git commit -m 'test: record local source-bound acceptance'
node --experimental-strip-types scripts/acceptance/local.mts push
```

`run` executes fmt, strict Clippy, the real namespace bootstrap first, every
remaining workspace test target through the portable command, then doctests.
This is the Linux profile: the existing `windows-dpapi-live` target requires a
Windows PE and interactive user profile and is explicitly recorded NOT EXECUTED.
Its ordinary contract tests still run. Do not count that native case as passed;
the receipt requires the exact declared exception, not an arbitrary skip list.
The early bootstrap catches capability/mount failures before expensive suites.
Failure, timeout, ignored/filtered tests,
empty evidence or source changes prevents receipt creation. Logs remain ignored
under `.local/acceptance` with private permissions. No command pushes on failure.
The receipt contains only the exact source commit, source digest, command results,
test totals, kernel version and log hashes; no hostname, account identity, paths,
credentials or log bodies. Commit it without editing source, then `push` checks
the clean tree and digest again before pushing the current branch to origin.
It never creates tags or publishes packages. Direct `git push` remains possible;
this is an owner workflow, not a machine-wide hook or a new GitHub permission rule.

## Remote acceptance and trust

The SHA256 digest covers the canonical Git path/mode/blob manifest of every
tracked regular file except evidence receipts, checked at both source and head;
changing code, tests, workflow or this policy invalidates older evidence. The
recorded source commit must be an ancestor of the reviewed head. CI detects stale,
missing or incomplete evidence and runs all portable targets. It explicitly marks
`production_bootstrap` as not executed on the hosted runner; it is never ignored
inside a supposedly complete remote test invocation.

A JSON receipt is a maintainer's execution claim, not an independently signed
remote execution attestation. Hashes bind source/log bytes but do not prove that
execution happened. A maintainer must review the exact source and actual local
logs before normal PR merge. Do not auto-merge based on receipt presence or treat
untrusted submitted receipts as proof. This does not change repository protection,
approvals or signing configuration. Preserve existing evidence after merge;
source changes require a new local run. Publication remains a separate gate.

Use a normal merge commit for the source/evidence PR, preserving the tested source
commit as an ancestor. Squash/rebase would break that ancestry and is rejected;
no repository setting is changed to enforce a different merge policy.
Test dependencies are optimized through Cargo's test profile to avoid slow debug
cryptography. Workspace debug assertions and production build profiles remain
unchanged; test failure/ignored filtering rules are not relaxed.
