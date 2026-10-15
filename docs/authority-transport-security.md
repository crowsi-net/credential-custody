# Authority transport security source

This source candidate admits only `crowsi-net/crowsi-transport-foundation@30bcd1e14cad837755cfc524097ca6133d32c8d3`,
authority transport 0.2.0 with Rustls 0.23.46. Normal dependencies remain exact
official registry dependencies. This one immutable public source exception avoids
the vulnerable exact TLS dependency in published 0.1.0 until its owner completes
the official 0.2.0 registry release (coela-oss/coela-release#123).

The public Rust signing key type changes from Dalek 2 to Dalek 3. This consumer
uses the current type directly; no key conversion, legacy decoder or permissive
verifier is introduced. Wire authorization and opaque envelopes stay unchanged.
No sibling checkout, private repository, moving branch/tag or fallback is used.
The committed lock binds the dependency to the full reviewed revision.

After official archive/provenance acceptance, replace this exception with the
accepted registry crate and regenerate the consumer lock. Source CI is not a
registry publication or native/device activation receipt. Real trust files and
credentials are untouched.
