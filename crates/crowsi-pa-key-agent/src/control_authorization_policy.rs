use crowsi_local_control_bridge::{BridgeAction, ControlRequestV1};

use crate::{PaKeyError, Result};

pub(crate) struct ControlAuthorizationPolicy {
    pub(crate) operation: &'static str,
    pub(crate) workload_id: &'static str,
    pub(crate) actor_profile_id: &'static str,
}

/// Narrows PA issuance to reviewed local-control operations.
pub(crate) fn require(request: &ControlRequestV1) -> Result<ControlAuthorizationPolicy> {
    let policy = match request.action {
        BridgeAction::EnrollCredential if credential_enrollment(request) => {
            ControlAuthorizationPolicy {
                operation: "credential-enroll",
                workload_id: "spiffe://crowsi/local/credential-agent",
                actor_profile_id: "profile-credential-operator",
            }
        }
        _ => return Err(PaKeyError::Authorization),
    };
    if request.schema != "crowsi://local-control/request/v1"
        || !identifier(&request.request_id)
        || !digest(&request.body_sha256)
    {
        return Err(PaKeyError::Authorization);
    }
    Ok(policy)
}

fn credential_enrollment(value: &ControlRequestV1) -> bool {
    value.purpose == "credential-enrollment"
        && value.resource.len() == 75
        && value.resource.starts_with("credential-")
        && value.resource[11..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 240
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b":._/-".contains(&byte))
}

fn digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
