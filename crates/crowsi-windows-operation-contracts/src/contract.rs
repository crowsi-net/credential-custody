use serde::{Deserialize, Serialize};

pub const OPERATION_REQUEST_SCHEMA: &str = "crowsi://platform-custody/operation-request/v3";
pub const OPERATION_AUTHORIZATION_SCHEMA: &str =
    "crowsi://platform-custody/operation-authorization/v3";
pub const OPERATION_RESPONSE_SCHEMA: &str = "crowsi://platform-custody/operation-response/v3";
pub const OPERATION_AUTHORIZATION_ISSUER: &str = "crowsi-policy-administrator";
pub const OPERATION_AUTHORIZATION_AUDIENCE: &str = "crowsi-windows-custody-provider";
pub const OPERATION_SERVICE_ID: &str = "service:crowsi";
pub const OPERATION_WORKLOAD_ID: &str = "workload:crowsi-windows-custody";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationOnlyCredentialClass {
    RsaSigningKey,
    Ed25519SigningKey,
    ProviderOperation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SigningAlgorithm {
    RsaPkcs1Sha256,
    Ed25519,
}

impl SigningAlgorithm {
    /// Returns the closed protocol name for this signing algorithm.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RsaPkcs1Sha256 => "rsa-pkcs1-sha256",
            Self::Ed25519 => "ed25519",
        }
    }
}

mod binding;
mod request;
mod response;
pub use binding::{OperationBinding, PaOperationAuthorization};
pub use request::{OperationOnlyAction, OperationOnlyRequest};
pub use response::{OperationOnlyResponse, OperationOnlyResult};
