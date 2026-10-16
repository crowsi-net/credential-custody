use crowsi_device_credential_agent::canonical_signed_document;
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
#[path = "namespace.rs"]
mod namespace;

pub struct Installation {
    pub root: PathBuf,
    base: PathBuf,
}
impl Installation {
    pub fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("isolated bootstrap setup")
            .as_nanos();
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
        fs::create_dir_all(&base).expect("isolated bootstrap setup");
        let base = base.join(format!("production-agent-{}-{nonce}", std::process::id()));
        fs::create_dir(&base).expect("isolated bootstrap setup");
        fs::set_permissions(&base, fs::Permissions::from_mode(0o700))
            .expect("isolated bootstrap setup");
        let root = base.join("root");
        let this = Self { root, base };
        for name in [
            "",
            "etc",
            "etc/crowsi",
            "etc/crowsi/device-credential-agent",
            "case",
            "case/state",
            "case/anchor",
            "app",
        ] {
            fs::create_dir(this.root.join(name)).expect("isolated bootstrap setup");
            fs::set_permissions(this.root.join(name), fs::Permissions::from_mode(0o700))
                .expect("isolated bootstrap setup");
        }
        fs::copy(
            env!("CARGO_BIN_EXE_crowsi-device-credential-agent"),
            this.root.join("app/agent"),
        )
        .expect("isolated bootstrap setup");
        let key = SigningKey::from_bytes(&[1; 32]);
        this.write("etc/crowsi/device-credential-agent/root-trust.json", &serde_json::to_vec(&json!({
            "schema":"crowsi://device-credential-agent/root-trust/v1", "configuration_key_id":"config-key",
            "configuration_public_key_hex":hex::encode(key.verifying_key().to_bytes())})).expect("isolated bootstrap setup"), 0o644);
        let sender = serde_json::to_vec(
            &json!({"schema":"crowsi://device-credential-agent/session-sender-key/v1",
            "key_id":"session-sender-key", "private_key_hex":hex::encode([7;32])}),
        )
        .expect("isolated bootstrap setup");
        this.write("case/sender.json", &sender, 0o600);
        let mut config: Value = crate::management_support_config::value(
            std::path::Path::new("/case"),
            &SigningKey::from_bytes(&[2; 32]),
        );
        config["session_sender_private_key_path"] = "/case/sender.json".into();
        config["session_sender_private_key_sha256"] =
            format!("sha256:{:x}", Sha256::digest(&sender)).into();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("isolated bootstrap setup")
            .as_secs();
        config["issued_at_epoch_s"] = (now - 1).into();
        config["expires_at_epoch_s"] = (now + 600).into();
        let canonical = canonical_signed_document("CROWSI-DEVICE-CREDENTIAL-CONFIG-V6", &config)
            .expect("isolated bootstrap setup");
        config["signature"] = hex::encode(key.sign(&canonical).to_bytes()).into();
        this.write(
            "case/config.json",
            &serde_json::to_vec(&config).expect("isolated bootstrap setup"),
            0o600,
        );
        this
    }
    pub fn write(&self, name: &str, bytes: &[u8], mode: u32) {
        fs::write(self.root.join(name), bytes).expect("isolated bootstrap setup");
        fs::set_permissions(self.root.join(name), fs::Permissions::from_mode(mode))
            .expect("isolated bootstrap setup");
    }
}
impl Drop for Installation {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.base).expect("remove only owned test directory");
    }
}
