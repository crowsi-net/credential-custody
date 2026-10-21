use std::{fs, os::unix::fs::PermissionsExt};

use crowsi_credential_broker::MemoryStore;
use crowsi_local_control_bridge::{BridgeAction, ControlRequestV1};
use crowsi_pa_key_agent::{
    PaKeyError, create_credential_challenge, initialize, issue_credential_authorization,
};
use p256::ecdsa::SigningKey;
use tempfile::tempdir;

use crate::support::{authorization_files, identity_evidence, owner_json, pa_binding, register};

#[test]
fn retired_provider_requests_cannot_create_challenges_or_issue_envelopes() {
    let root = tempdir().expect("private root");
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700)).expect("mode");
    let store = MemoryStore::default();
    let state = root.path().join("pa.json");
    let pa = initialize(&store, &state, "memory-test").expect("PA");
    let credential = root.path().join("passkey.json");
    let signing = SigningKey::from_bytes((&[31_u8; 32]).into()).expect("P-256 key");
    let binding = pa_binding(&pa.public_key_hex);
    register(&credential, root.path(), &signing, &binding);
    let (identity, identity_status, identity_trust) = identity_evidence(root.path(), "retired");
    let challenge = root.path().join("challenge.json");
    let assertion = root.path().join("assertion.json");
    let output = root.path().join("authorization.json");
    let request_path = root.path().join("request.json");
    let state_before = fs::read(&state).expect("state");
    let credential_before = fs::read(&credential).expect("credential");
    let status_before = fs::read(&identity_status).expect("identity status");
    for (resource, purpose) in [
        (
            "crowsi://credentials/github-app/example-observer",
            "github-app-jwt-signing",
        ),
        ("crowsi://credentials/github-app", "github-app-jwt-signing"),
        (
            "crowsi://credentials/github-app/example-observer",
            "repository-write",
        ),
    ] {
        owner_json(
            &request_path,
            &ControlRequestV1 {
                schema: "crowsi://local-control/request/v1".into(),
                request_id: "request-retired-observer".into(),
                action: BridgeAction::ObserveProvider,
                resource: resource.into(),
                purpose: purpose.into(),
                body_sha256: format!("sha256:{}", "b".repeat(64)),
            },
        );
        let result = create_credential_challenge(
            authorization_files(
                &state,
                &credential,
                &request_path,
                &identity,
                &identity_status,
                &identity_trust,
            ),
            &challenge,
            &binding,
            "http://localhost:4213",
        );
        assert!(matches!(result, Err(PaKeyError::Authorization)));
        assert!(!challenge.exists());
        // Even an earlier ceremony's files must not be consumed after retirement.
        owner_json(&challenge, &serde_json::json!({"legacy": "challenge"}));
        owner_json(&assertion, &serde_json::json!({"legacy": "assertion"}));
        let challenge_before = fs::read(&challenge).expect("challenge");
        let assertion_before = fs::read(&assertion).expect("assertion");
        let result = issue_credential_authorization(
            &store,
            authorization_files(
                &state,
                &credential,
                &request_path,
                &identity,
                &identity_status,
                &identity_trust,
            ),
            &challenge,
            &assertion,
            &output,
        );
        assert!(matches!(result, Err(PaKeyError::Authorization)));
        assert!(!output.exists());
        assert_eq!(
            fs::read(&challenge).expect("challenge retained"),
            challenge_before
        );
        assert_eq!(
            fs::read(&assertion).expect("assertion retained"),
            assertion_before
        );
        assert_eq!(fs::read(&state).expect("state retained"), state_before);
        assert_eq!(
            fs::read(&credential).expect("credential retained"),
            credential_before
        );
        assert_eq!(
            fs::read(&identity_status).expect("status retained"),
            status_before
        );
        fs::remove_file(&challenge).expect("synthetic challenge cleanup");
        fs::remove_file(&assertion).expect("synthetic assertion cleanup");
    }
}
