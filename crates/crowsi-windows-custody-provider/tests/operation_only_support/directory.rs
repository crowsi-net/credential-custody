//! Test-only nonce state with the same private parent policy as production.
#[cfg(unix)]
pub fn tempdir() -> std::io::Result<tempfile::TempDir> {
    use std::{fs, os::unix::fs::PermissionsExt};
    tempfile::Builder::new()
        .permissions(fs::Permissions::from_mode(0o700))
        .tempdir()
}

#[cfg(not(unix))]
pub fn tempdir() -> std::io::Result<tempfile::TempDir> {
    tempfile::tempdir()
}
