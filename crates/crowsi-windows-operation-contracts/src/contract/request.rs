//! Closed request documents for a single authorized custody operation.

use super::{OperationOnlyCredentialClass, PaOperationAuthorization, SigningAlgorithm};
use ihat_identity_assertion_contracts::CurrentDeviceStatusV1;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum OperationOnlyAction {
    Sign {
        algorithm: SigningAlgorithm,
        digest_sha256: String,
    },
    ProviderOperation {
        provider: String,
        operation: String,
        input_digest_sha256: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationOnlyRequest {
    pub schema: String,
    pub request_id: String,
    pub credential_id: String,
    pub expected_revision: String,
    pub credential_class: OperationOnlyCredentialClass,
    pub action: OperationOnlyAction,
    pub current_device_status: CurrentDeviceStatusV1,
    pub pa_authorization: PaOperationAuthorization,
}
