//! Canonical identities must work with both Unix roots and Windows drive prefixes.
use crowsi_windows_custody_provider::HelperIdentity;
use sha2::{Digest, Sha256};
use std::fs;

#[test]
fn canonical_absolute_helper_is_pinned_and_detects_replacement() {
    let root = tempfile::tempdir().expect("private fixture directory");
    let helper = root.path().join("helper.exe");
    fs::write(&helper, b"synthetic executable bytes").expect("fixture");
    let helper = helper.canonicalize().expect("canonical helper");
    let digest = format!(
        "sha256:{}",
        hex::encode(Sha256::digest(b"synthetic executable bytes"))
    );
    assert!(HelperIdentity::new(&helper, &digest).is_ok());
    fs::write(&helper, b"replaced bytes").expect("replace fixture");
    assert!(HelperIdentity::new(&helper, &digest).is_err());
}

#[test]
fn relative_paths_and_malformed_digests_remain_rejected() {
    assert!(HelperIdentity::new("helper.exe", format!("sha256:{}", "0".repeat(64))).is_err());
    let root = tempfile::tempdir().expect("private fixture directory");
    let helper = root.path().join("helper.exe");
    fs::write(&helper, b"synthetic executable bytes").expect("fixture");
    assert!(HelperIdentity::new(helper.canonicalize().expect("canonical"), "untrusted").is_err());
}
