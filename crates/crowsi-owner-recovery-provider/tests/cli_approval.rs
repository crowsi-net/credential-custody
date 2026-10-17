mod support;
use crowsi_owner_recovery_provider::{descriptor, parse};
use ihat_identity_assertion_contracts::{
    SignedEvidenceBinding, SignedEvidenceV1, VerificationRole, verify_signed_evidence_at,
};
use support::{phrase, run};
#[test]
fn ordinary_provider_approval_is_exact_root_and_operation_bound() {
    let mnemonic = parse(&phrase()).expect("published synthetic vector");
    let root = descriptor(&mnemonic);
    let binding = "ab".repeat(32);
    let output = run(
        &[
            "approve-stdin",
            "recovery-proof-1",
            &binding,
            &root.key_id,
            "100",
            "120",
        ],
        phrase().as_bytes(),
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let evidence: SignedEvidenceV1 =
        serde_json::from_slice(&output.stdout).expect("typed evidence");
    let expected = SignedEvidenceBinding {
        role: VerificationRole::RecoveryApproval,
        proof_id: "recovery-proof-1",
        binding_sha256: &binding,
    };
    verify_signed_evidence_at(
        &evidence,
        &expected,
        &root.key_id,
        &root.public_key_hex,
        110,
    )
    .expect("real iHAT verification");
    let other = crowsi_owner_recovery_provider::generate().expect("synthetic independent root");
    let independent = descriptor(&other.mnemonic);
    assert!(
        verify_signed_evidence_at(
            &evidence,
            &expected,
            &independent.key_id,
            &independent.public_key_hex,
            110
        )
        .is_err()
    );
    let foreign_binding = "cd".repeat(32);
    let foreign = SignedEvidenceBinding {
        binding_sha256: &foreign_binding,
        ..expected
    };
    assert!(
        verify_signed_evidence_at(&evidence, &foreign, &root.key_id, &root.public_key_hex, 110)
            .is_err()
    );
    assert!(
        verify_signed_evidence_at(
            &evidence,
            &expected,
            &root.key_id,
            &root.public_key_hex,
            121
        )
        .is_err()
    );
}
#[test]
fn stale_excessive_or_foreign_key_requests_produce_no_approval() {
    let root = descriptor(&parse(&phrase()).expect("synthetic vector"));
    let binding = "ab".repeat(32);
    for (key, issued, expires) in [
        (&root.key_id[..], "100", "100"),
        (&root.key_id, "100", "999999"),
        ("foreign-root", "100", "120"),
    ] {
        let output = run(
            &[
                "approve-stdin",
                "recovery-proof-1",
                &binding,
                key,
                issued,
                expires,
            ],
            phrase().as_bytes(),
        );
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert_eq!(
            output.stderr,
            b"crowsi-owner-recovery-provider: approval-invalid\n"
        );
    }
    let output = run(
        &[
            "create",
            "/unused-synthetic-descriptor",
            "recovery:1",
            "owner:1",
            "provider:1",
            "1",
            "100",
        ],
        b"",
    );
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"crowsi-owner-recovery-provider: owner-display-terminal-required\n"
    );
}
