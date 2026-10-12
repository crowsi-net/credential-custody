#![cfg(unix)]

#[path = "fixtures/native_build.rs"]
mod native_build;

use std::{fs, time::Duration};

use crowsi_windows_custody_provider::{CustodyClient, HelperIdentity};
use sha2::{Digest, Sha256};
use tempfile::tempdir;

const SECRET: &[u8] = b"credential-material";

#[test]
fn put_and_get_keep_secret_out_of_control_json() {
    let directory = tempdir().expect("temporary directory");
    let helper = directory.path().join("provider.exe");
    native_build::build(&helper);
    let bytes = fs::read(&helper).expect("helper bytes");
    let digest = format!("sha256:{}", hex::encode(Sha256::digest(bytes)));
    let identity = HelperIdentity::new(helper, digest).expect("helper identity");
    let client = CustodyClient::new(identity, "coela.crowsi.credentials", Duration::from_secs(2))
        .expect("client");
    let put = client
        .put("request-put", "github.app.pem", None, SECRET)
        .expect("put exchange");
    let revision = put
        .response
        .metadata()
        .expect("put metadata")
        .revision()
        .to_owned();
    assert!(revision.starts_with("rev1:"));
    assert_ne!(
        revision,
        format!("sha256:{}", hex::encode(Sha256::digest(SECRET)))
    );
    assert!(
        !serde_json::to_vec(&put.response)
            .expect("response JSON")
            .windows(SECRET.len())
            .any(|window| window == SECRET)
    );
    let get = client
        .get("request-get", "github.app.pem", revision)
        .expect("get exchange");
    get.expose_secret(|secret| assert_eq!(secret, Some(SECRET)));
}
