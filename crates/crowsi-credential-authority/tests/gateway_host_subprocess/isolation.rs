#[path = "isolation_assertions.rs"]
mod assertions;
use assertions::{entry, rejected};
use crowsi_credential_authority::{
    DeviceId, OpaqueOwnerRef, RevocationCause,
    test_support::{CredentialAuthority, FileAuthorityStore, StepUpProof},
};
use crowsi_credential_authority_contracts::{
    EndpointManagementEnvelopeV2, EndpointManagementEvidenceV2,
};
use serde_json::Value;

use crate::{authority, config, fixture::Fixture, identity::*, process, process_run};

#[test]
fn device_a_epoch_change_survives_host_restart_without_changing_device_b() {
    let fixture = Fixture::new();
    authority::provision(&fixture);
    let documents = config::write(&fixture);
    process_run::initialize(&fixture, &documents);
    let before: Value =
        serde_json::from_slice(&process::peer_head(&fixture, &documents).1).expect("initial head");
    let store = FileAuthorityStore::open_anchored(&fixture.store, &fixture.authority_anchor)
        .expect("existing store");
    let mut owner = CredentialAuthority::with_store(store, fixture.now * 1_000);
    let account = OpaqueOwnerRef::parse(OWNER).expect("owner");
    owner
        .revoke_device(
            &account,
            &DeviceId::parse(DEVICE_A).expect("A"),
            RevocationCause::DeviceCompromised,
            StepUpProof::verified(
                account.clone(),
                DeviceId::parse(DEVICE_B).expect("B"),
                fixture.now * 1_000,
                (fixture.now + 30) * 1_000,
            ),
        )
        .expect("revoke A epoch");
    let after: Value = serde_json::from_slice(&process::peer_head(&fixture, &documents).1)
        .expect("new child sees durable head");
    assert_eq!(entry(&after, DEVICE_A)["device_revocation_epoch"], 8);
    assert_eq!(entry(&before, DEVICE_A)["device_revocation_epoch"], 7);
    assert_eq!(entry(&before, DEVICE_B), entry(&after, DEVICE_B));
}

#[test]
fn copied_a_identity_cannot_be_used_as_registered_peer_b() {
    let fixture = Fixture::new();
    authority::provision(&fixture);
    let documents = config::write(&fixture);
    process_run::initialize(&fixture, &documents);
    let mut copied: Value =
        serde_json::from_slice(&process::snapshot_wire(&fixture)).expect("wire");
    copied["peer"]["device_id"] = DEVICE_B.into();
    copied["peer"]["certificate_sha256"] = format!("sha256:{}", "b".repeat(64)).into();
    copied["peer"]["request_key_id"] = "request-b".into();
    copied["peer"]["request_public_key_hex"] = crate::fixture::public(13).into();
    let output = process_run::try_run(
        &fixture,
        &documents,
        "management-v2",
        &serde_json::to_vec(&copied).expect("copied request"),
    );
    rejected(&output);
    let legitimate = process_run::run(
        &fixture,
        &documents,
        "management-v2",
        &process::snapshot_wire(&fixture),
    );
    assert!(!legitimate.is_empty(), "rejection must not retire A");
}

#[test]
fn modified_signed_identity_response_is_rejected_by_the_real_host() {
    let fixture = Fixture::new();
    authority::provision(&fixture);
    let documents = config::write(&fixture);
    process_run::initialize(&fixture, &documents);
    let mut wire: Value = serde_json::from_slice(&process::snapshot_wire(&fixture)).expect("wire");
    let payload = hex::decode(wire["payload_hex"].as_str().expect("payload")).expect("hex");
    let mut payload: EndpointManagementEnvelopeV2 =
        serde_json::from_slice(&payload).expect("envelope");
    let EndpointManagementEvidenceV2::Passive { identity_exchange } = &mut payload.evidence else {
        panic!("passive evidence")
    };
    identity_exchange.response.signature = "00".repeat(64);
    wire["payload_hex"] = hex::encode(serde_json::to_vec(&payload).expect("payload")).into();
    rejected(&process_run::try_run(
        &fixture,
        &documents,
        "management-v2",
        &serde_json::to_vec(&wire).expect("wire"),
    ));
    let (_, head) = process::peer_head(&fixture, &documents);
    let head: Value = serde_json::from_slice(&head).expect("head");
    assert_eq!(entry(&head, DEVICE_A)["device_revocation_epoch"], 7);
    assert_eq!(entry(&head, DEVICE_B)["device_revocation_epoch"], 7);
}

#[test]
fn reading_an_uninitialized_authority_does_not_register_devices() {
    let fixture = Fixture::new();
    let documents = config::write(&fixture);
    rejected(&process_run::try_run(
        &fixture,
        &documents,
        "management-v2",
        &process::snapshot_wire(&fixture),
    ));
    assert!(FileAuthorityStore::open_anchored(&fixture.store, &fixture.authority_anchor).is_err());
}
