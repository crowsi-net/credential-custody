use crowsi_credential_broker::{
    Broker, BrokerError, CredentialEntry, CredentialStore, LinuxCustodyStore, SecretRef,
    SecretScope, SecretValue,
};

use super::support;

fn reference(id: &str) -> SecretRef {
    SecretRef::new(
        id,
        SecretScope::new("synthetic", "keyring", "api-call", "worker").expect("scope"),
    )
    .expect("reference")
}

fn entry(reference: SecretRef, secret: &[u8]) -> CredentialEntry {
    CredentialEntry::new(
        reference,
        ["api.example.test".into()],
        SecretValue::new(secret.to_vec()).expect("secret"),
    )
    .expect("entry")
}

pub fn exercise(store: &LinuxCustodyStore) {
    let reference = reference("persisted");
    let first = entry(reference.clone(), b"synthetic-persisted-secret");
    let revision = first.metadata().revision().as_str().to_owned();
    store.put(first).expect("verified write");
    let read = store.get(&reference, &revision).expect("read own revision");
    read.into_secret()
        .expose(|value| assert_eq!(value, b"synthetic-persisted-secret"));
    assert!(matches!(
        store.put(store.get(&reference, &revision).expect("entry")),
        Err(BrokerError::CredentialChanged)
    ));
    store
        .put(entry(reference.clone(), b"synthetic-persisted-secret"))
        .expect("rotate revision");
    assert!(matches!(
        store.get(&reference, &revision),
        Err(BrokerError::CredentialChanged)
    ));
    let other = super::cases::reference("other-workspace");
    assert!(matches!(store.metadata(&other), Err(BrokerError::NotFound)));
    store
        .put(entry(other.clone(), b"temporary-synthetic"))
        .expect("other write");
    store.delete(&other).expect("delete exact slot");
    assert!(matches!(store.metadata(&other), Err(BrokerError::NotFound)));
    assert!(matches!(store.delete(&other), Err(BrokerError::NotFound)));
    let reopened = LinuxCustodyStore::connect("default").expect("independent connection");
    assert_eq!(
        reopened.metadata(&reference).expect("metadata"),
        store.metadata(&reference).expect("metadata")
    );
    super::integrity::verify(store, &reference);
}

pub fn restart(store: &LinuxCustodyStore) {
    let reference = reference("persisted");
    let metadata = store
        .metadata(&reference)
        .expect("persisted after daemon restart");
    store
        .get(&reference, metadata.revision().as_str())
        .expect("persisted value")
        .into_secret()
        .expose(|value| assert_eq!(value, b"synthetic-persisted-secret"));
    let owned = std::sync::Arc::new(LinuxCustodyStore::connect("default").expect("connection"));
    let broker = Broker::with_clock(owned, support::clock(100).0);
    let request = support::request(reference.clone(), "worker", "api.example.test", 30);
    let lease = broker
        .issue_bound(request, support::binding())
        .expect("bound lease");
    store.lock().expect("provider lock");
    assert!(store.is_locked().expect("locked status"));
    assert!(matches!(
        store.get(&reference, metadata.revision().as_str()),
        Err(BrokerError::BackendDenied)
    ));
    assert!(matches!(
        store.put(entry(reference.clone(), b"must-not-write")),
        Err(BrokerError::BackendDenied)
    ));
    assert!(matches!(
        store.delete(&reference),
        Err(BrokerError::BackendDenied)
    ));
    assert!(
        broker
            .consume_bound(
                lease.token(),
                "worker",
                "api.example.test",
                &support::binding()
            )
            .is_err()
    );
}
