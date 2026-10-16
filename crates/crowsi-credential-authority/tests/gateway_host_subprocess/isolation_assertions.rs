use serde_json::Value;

pub(super) fn entry<'a>(head: &'a Value, device: &str) -> &'a Value {
    head["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .find(|entry| entry["device_id"] == device)
        .expect("registered peer")
}

pub(super) fn rejected(output: &std::process::Output) {
    assert!(!output.status.success(), "invalid request was accepted");
    assert!(
        output.stdout.is_empty(),
        "failure must not return a projection"
    );
    assert!(!output.stderr.is_empty(), "refusal must be explicit");
}
