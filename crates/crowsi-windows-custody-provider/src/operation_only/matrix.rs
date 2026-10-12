use super::{OperationOnlyAction, OperationOnlyCredentialClass, SigningAlgorithm};

pub(crate) fn valid_matrix(
    class: OperationOnlyCredentialClass,
    action: &OperationOnlyAction,
) -> bool {
    match (class, action) {
        (
            OperationOnlyCredentialClass::RsaSigningKey,
            OperationOnlyAction::Sign {
                algorithm: SigningAlgorithm::RsaPkcs1Sha256,
                ..
            },
        )
        | (
            OperationOnlyCredentialClass::Ed25519SigningKey,
            OperationOnlyAction::Sign {
                algorithm: SigningAlgorithm::Ed25519,
                ..
            },
        ) => true,
        (
            OperationOnlyCredentialClass::ProviderOperation,
            OperationOnlyAction::ProviderOperation {
                provider,
                operation,
                ..
            },
        ) => provider == "github" && operation == "sign-app-jwt",
        _ => false,
    }
}

pub(crate) use crowsi_windows_operation_contracts::validate_operation_result as valid_result;

pub(crate) fn action_binding(action: &OperationOnlyAction) -> String {
    match action {
        OperationOnlyAction::Sign { algorithm, .. } => format!("sign:{}", algorithm.as_str()),
        OperationOnlyAction::ProviderOperation {
            provider,
            operation,
            ..
        } => format!("provider-operation:{provider}:{operation}"),
    }
}
