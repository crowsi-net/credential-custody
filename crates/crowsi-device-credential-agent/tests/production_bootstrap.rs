#![recursion_limit = "256"]
mod management_support_config;
mod management_support_route;
mod management_support {
    pub const NOW: u64 = 1_900_000_000;
}
#[path = "production/bootstrap.rs"]
mod bootstrap;
use bootstrap::Installation;

#[test]
fn ordinary_agent_uses_installed_trust_and_refuses_repeat_initialization() {
    let installed = Installation::new();
    let first = installed.run("initialize-once");
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(first.stdout.is_empty());
    let binding = std::fs::read(installed.root.join("case/state/binding.json"))
        .expect("isolated bootstrap setup");
    let again = installed.run("initialize-once");
    assert!(
        !again.status.success(),
        "initialization must not reset existing state"
    );
    assert!(again.stdout.is_empty());
    assert_eq!(
        std::fs::read(installed.root.join("case/state/binding.json"))
            .expect("isolated bootstrap setup"),
        binding
    );
}

#[test]
fn invalid_installed_trust_cannot_create_endpoint_state() {
    let installed = Installation::new();
    installed.write(
        "etc/crowsi/device-credential-agent/root-trust.json",
        b"{}",
        0o644,
    );
    let refused = installed.run("initialize-once");
    assert!(!refused.status.success());
    assert!(refused.stdout.is_empty());
    assert_eq!(
        std::fs::read_dir(installed.root.join("case/state"))
            .expect("isolated bootstrap setup")
            .count(),
        0
    );
}

#[test]
fn writable_installed_trust_is_rejected_without_initialization() {
    use std::os::unix::fs::PermissionsExt;
    let installed = Installation::new();
    let path = installed
        .root
        .join("etc/crowsi/device-credential-agent/root-trust.json");
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o666))
        .expect("unsafe test mode");
    let refused = installed.run("initialize-once");
    assert!(!refused.status.success());
    assert!(refused.stdout.is_empty());
    assert_eq!(
        std::fs::read_dir(installed.root.join("case/state"))
            .expect("state")
            .count(),
        0
    );
}

#[test]
fn changed_configuration_without_a_new_signature_is_rejected() {
    let installed = Installation::new();
    let wire = std::fs::read(installed.root.join("case/config.json")).expect("config");
    let mut value: serde_json::Value = serde_json::from_slice(&wire).expect("typed test document");
    value["configuration_generation"] = 2.into();
    installed.write(
        "case/config.json",
        &serde_json::to_vec(&value).expect("modified document"),
        0o600,
    );
    let refused = installed.run("initialize-once");
    assert!(!refused.status.success());
    assert!(refused.stdout.is_empty());
    assert_eq!(
        std::fs::read_dir(installed.root.join("case/state"))
            .expect("state")
            .count(),
        0
    );
}
