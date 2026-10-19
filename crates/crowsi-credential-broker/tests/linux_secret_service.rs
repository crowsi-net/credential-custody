#![cfg(target_os = "linux")]

#[path = "linux_service_cases.rs"]
mod cases;
#[path = "linux_service_integrity.rs"]
mod integrity;
#[path = "support/mod.rs"]
mod support;

use crowsi_credential_broker::{BrokerError, LinuxCustodyStore};

#[test]
fn reject_invalid_or_ephemeral_collection_before_connecting() {
    for alias in ["", "session", "../default", "a b", &"x".repeat(65)] {
        assert!(matches!(
            LinuxCustodyStore::connect(alias),
            Err(BrokerError::InvalidField("collection_alias"))
        ));
    }
}

#[test]
#[ignore = "requires scripts/acceptance/linux-keyring.sh isolated bus and synthetic keyring"]
fn isolated_provider_acceptance() {
    assert_eq!(std::env::var("CROWSI_ISOLATED_KEYRING").as_deref(), Ok("1"));
    let phase = std::env::var("CROWSI_KEYRING_PHASE").expect("explicit acceptance phase");
    if phase == "unavailable" {
        assert!(matches!(
            LinuxCustodyStore::connect("default"),
            Err(BrokerError::BackendUnavailable)
        ));
        return;
    }
    let store = LinuxCustodyStore::connect("default").expect("isolated keyring service");
    assert!(!store.is_locked().expect("provider status"));
    store
        .unlock()
        .expect("already unlocked provider, no password to Crowsi");
    match phase.as_str() {
        "seed" => cases::exercise(&store),
        "restart" => cases::restart(&store),
        _ => panic!("unknown acceptance phase"),
    }
}
