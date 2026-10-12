use ed25519_dalek::{Signer, SigningKey};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub(crate) fn key(value: u8) -> SigningKey {
    SigningKey::from_bytes(&[value; 32])
}

pub(crate) fn public(value: u8) -> String {
    hex::encode(key(value).verifying_key().to_bytes())
}

pub(crate) fn digest(value: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(value))
}

pub(crate) fn sign(domain: &str, value: &mut Value, key: &SigningKey) -> Vec<u8> {
    value["signature"] = Value::String(String::new());
    let mut unsigned = value.clone();
    unsigned
        .as_object_mut()
        .expect("object")
        .remove("signature");
    let body = serde_json::to_vec(&unsigned).expect("body");
    let payload = [
        domain.as_bytes(),
        b"\n",
        body.len().to_string().as_bytes(),
        b"\n",
        &body,
    ]
    .concat();
    value["signature"] = hex::encode(key.sign(&payload).to_bytes()).into();
    serde_json::to_vec(value).expect("signed wire")
}
