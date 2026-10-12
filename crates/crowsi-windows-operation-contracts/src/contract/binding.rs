//! Closed binding documents for a single authorized custody operation.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationBinding {
    pub service_id: String,
    pub pairwise_subject: String,
    pub device_id: String,
    pub device_proof_key_ref: String,
    pub session_ref: String,
    pub device_posture: String,
    pub device_posture_revision: u64,
    pub subject_revocation_epoch: u64,
    pub service_revocation_epoch: u64,
    pub device_revocation_epoch: u64,
    pub session_revocation_epoch: u64,
    pub workload_id: String,
    pub audience: String,
    pub action: String,
    pub request_digest_sha256: String,
    pub nonce: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PaOperationAuthorization {
    pub schema: String,
    pub issuer: String,
    pub key_id: String,
    pub binding: OperationBinding,
    pub issued_at_epoch_s: u64,
    pub expires_at_epoch_s: u64,
    pub signature_hex: String,
}
