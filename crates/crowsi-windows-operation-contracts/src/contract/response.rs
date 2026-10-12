//! Closed response documents for a single authorized custody operation.

use super::SigningAlgorithm;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum OperationOnlyResult {
    Signature {
        algorithm: SigningAlgorithm,
        value_hex: String,
    },
    ProviderResult {
        provider: String,
        operation: String,
        value_hex: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationOnlyResponse {
    pub schema: String,
    pub request_id: String,
    pub credential_id: String,
    pub revision: String,
    pub result: OperationOnlyResult,
    pub contains_secret_values: bool,
    pub secret_follows: bool,
}
