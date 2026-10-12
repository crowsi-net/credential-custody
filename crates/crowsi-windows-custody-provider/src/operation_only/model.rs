//! Shared operation wire types; backend-only execution context stays local.

pub use crowsi_windows_operation_contracts::{
    OPERATION_AUTHORIZATION_AUDIENCE, OPERATION_AUTHORIZATION_ISSUER,
    OPERATION_AUTHORIZATION_SCHEMA, OPERATION_REQUEST_SCHEMA, OPERATION_RESPONSE_SCHEMA,
    OPERATION_SERVICE_ID, OPERATION_WORKLOAD_ID, OperationBinding, OperationOnlyAction,
    OperationOnlyCredentialClass, OperationOnlyRequest, OperationOnlyResponse, OperationOnlyResult,
    PaOperationAuthorization, SigningAlgorithm,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationContext {
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
    pub now_epoch_s: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationCredentialMetadata {
    pub credential_id: String,
    pub revision: String,
    pub credential_class: OperationOnlyCredentialClass,
}
