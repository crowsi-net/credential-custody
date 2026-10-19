use crowsi_credential_broker::{BrokerError, CredentialStore, LinuxCustodyStore, SecretRef};
use oo7::dbus::Service;
use sha2::{Digest, Sha256};

pub fn verify(store: &LinuxCustodyStore, reference: &SecretRef) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("test runtime");
    runtime.block_on(async {
        let service = Service::encrypted().await.expect("test service");
        let collection = service
            .with_alias("default")
            .await
            .expect("alias lookup")
            .expect("collection");
        let attributes = [
            (
                "application".to_owned(),
                "crowsi-credential-broker-v1".to_owned(),
            ),
            (
                "slot".to_owned(),
                hex::encode(Sha256::digest(reference.canonical_id().as_bytes())),
            ),
        ];
        let item = collection
            .search_items(&attributes)
            .await
            .expect("slot")
            .pop()
            .expect("one item");
        let original = item.secret().await.expect("synthetic envelope");
        item.set_secret(oo7::Secret::blob(b"not-an-envelope"))
            .await
            .expect("corruption setup");
        assert!(matches!(
            store.metadata(reference),
            Err(BrokerError::InvalidEncoding)
        ));
        let mut wrong: serde_json::Value =
            serde_json::from_slice(original.as_bytes()).expect("synthetic JSON");
        wrong["reference"] = serde_json::Value::String("wrong-reference".into());
        item.set_secret(oo7::Secret::blob(
            serde_json::to_vec(&wrong).expect("synthetic JSON"),
        ))
        .await
        .expect("substitution setup");
        assert!(matches!(
            store.metadata(reference),
            Err(BrokerError::AccessDenied)
        ));
        item.set_secret(original.clone())
            .await
            .expect("restore synthetic envelope");
        let duplicate = collection
            .create_item("synthetic duplicate", &attributes, original, false, None)
            .await
            .expect("duplicate setup");
        assert!(matches!(
            store.metadata(reference),
            Err(BrokerError::AccessDenied)
        ));
        duplicate
            .delete(None)
            .await
            .expect("remove synthetic duplicate");
        assert!(store.metadata(reference).is_ok());
    });
}
