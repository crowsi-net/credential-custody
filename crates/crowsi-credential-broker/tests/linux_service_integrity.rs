use crowsi_credential_broker::{BrokerError, CredentialStore, LinuxCustodyStore, SecretRef};
use gio::prelude::*;
use libsecret::{
    Collection, CollectionFlags, Item, ItemCreateFlags, SearchFlags, Service, ServiceFlags, Value,
    prelude::*,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub fn verify(store: &LinuxCustodyStore, reference: &SecretRef) {
    let context = glib::MainContext::new();
    context
        .with_thread_default(|| {
            context.block_on(async {
                let service =
                    Service::open_future(Service::static_type(), None, ServiceFlags::OPEN_SESSION)
                        .await
                        .expect("test provider");
                let collection =
                    Collection::for_alias_future(Some(&service), "default", CollectionFlags::NONE)
                        .await
                        .expect("test alias")
                        .expect("collection");
                let slot = hex::encode(Sha256::digest(reference.canonical_id().as_bytes()));
                let attributes = HashMap::from([
                    ("application", "crowsi-credential-broker-v1"),
                    ("slot", slot.as_str()),
                ]);
                let item = collection
                    .search_future(None, attributes, SearchFlags::ALL)
                    .await
                    .expect("slot")
                    .pop()
                    .expect("item");
                item.load_secret_future().await.expect("synthetic secret");
                let original = item.secret().expect("synthetic envelope");
                item.set_secret_future(&Value::new("not-an-envelope", "application/json"))
                    .await
                    .expect("corruption setup");
                assert!(matches!(
                    store.metadata(reference),
                    Err(BrokerError::InvalidEncoding)
                ));
                assert!(matches!(
                    store.delete(reference),
                    Err(BrokerError::InvalidEncoding)
                ));
                let mut wrong: serde_json::Value =
                    serde_json::from_slice(&original.get()).expect("synthetic JSON");
                wrong["reference"] = serde_json::Value::String("wrong-reference".into());
                let encoded = serde_json::to_string(&wrong).expect("synthetic JSON");
                item.set_secret_future(&Value::new(&encoded, "application/json"))
                    .await
                    .expect("substitution");
                assert!(matches!(
                    store.metadata(reference),
                    Err(BrokerError::AccessDenied)
                ));
                assert!(matches!(
                    store.delete(reference),
                    Err(BrokerError::AccessDenied)
                ));
                item.set_secret_future(&original)
                    .await
                    .expect("restore synthetic envelope");
                let attrs = HashMap::from([
                    ("application".into(), "crowsi-credential-broker-v1"),
                    ("slot".into(), slot.as_str()),
                ]);
                let duplicate = Item::create_future(
                    &collection,
                    None,
                    attrs,
                    "synthetic duplicate",
                    &original,
                    ItemCreateFlags::NONE,
                )
                .await
                .expect("duplicate setup");
                assert!(matches!(
                    store.metadata(reference),
                    Err(BrokerError::AccessDenied)
                ));
                duplicate
                    .delete_future()
                    .await
                    .expect("remove synthetic duplicate");
                assert!(store.metadata(reference).is_ok());
            })
        })
        .expect("test thread context");
}
