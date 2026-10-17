mod support;
use support::{phrase, run};
fn verify(input: &[u8]) -> std::process::Output {
    run(
        &[
            "verify-stdin",
            "recovery:1",
            "owner:1",
            "crowsi:owner-recovery",
            "1",
            "100",
        ],
        input,
    )
}
#[test]
fn oversized_input_cannot_be_accepted_as_a_valid_truncated_prefix() {
    assert!(
        verify(phrase().as_bytes()).status.success(),
        "positive fixture"
    );
    let mut input = phrase().into_bytes();
    input.resize(1024, b'\n');
    input.push(b'X');
    let refused = verify(&input);
    assert!(
        !refused.status.success(),
        "provider accepted only a valid prefix of oversized input"
    );
    assert!(refused.stdout.is_empty());
    assert_eq!(
        refused.stderr,
        b"crowsi-owner-recovery-provider: secret-input-too-large\n"
    );
}

#[test]
fn exact_input_limit_and_crlf_are_accepted_but_invalid_or_trailing_data_are_refused() {
    let mut input = phrase().into_bytes();
    input.resize(1024, b'\n');
    assert!(verify(&input).status.success(), "exact byte budget");
    assert!(
        verify(format!("{}\r\n", phrase()).as_bytes())
            .status
            .success()
    );
    for input in [
        b"invalid synthetic phrase".to_vec(),
        vec![0xff],
        format!("{}\nPRIVATE_CANARY", phrase()).into_bytes(),
    ] {
        let refused = verify(&input);
        assert!(!refused.status.success());
        assert!(refused.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&refused.stderr).contains("PRIVATE_CANARY"));
    }
}
