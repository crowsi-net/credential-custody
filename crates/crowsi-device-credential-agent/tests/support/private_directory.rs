//! Owner-private test state; independent of the caller's umask and build-output modes.
use std::{fs, os::unix::fs::PermissionsExt};

pub(super) fn create(purpose: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(&format!(".crowsi-test-{purpose}-"))
        .permissions(fs::Permissions::from_mode(0o700))
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .expect("private test directory")
}
