use crate::{BrokerError, CredentialEntry, Result, SecretRef, custody_codec};
use gio::prelude::*;
use libsecret::{
    Collection, CollectionFlags, Item, SearchFlags, Service, ServiceFlags, prelude::*,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use zeroize::Zeroizing;

pub(super) struct Backend {
    pub(super) service: Service,
    pub(super) collection: Collection,
}

pub(super) fn provider_error(_: glib::Error) -> BrokerError {
    BrokerError::BackendUnavailable
}

pub(super) fn attributes(reference: &SecretRef) -> HashMap<String, String> {
    HashMap::from([
        ("application".into(), "crowsi-credential-broker-v1".into()),
        (
            "slot".into(),
            hex::encode(Sha256::digest(reference.canonical_id().as_bytes())),
        ),
    ])
}

impl Backend {
    pub(super) async fn connect(alias: &str) -> Result<Self> {
        let service =
            Service::open_future(Service::static_type(), None, ServiceFlags::OPEN_SESSION)
                .await
                .map_err(provider_error)?;
        let collection = Collection::for_alias_future(Some(&service), alias, CollectionFlags::NONE)
            .await
            .map_err(provider_error)?
            .ok_or(BrokerError::BackendUnavailable)?;
        Ok(Self {
            service,
            collection,
        })
    }

    pub(super) async fn locked(&self) -> Result<bool> {
        // Read live state rather than trusting a cached GObject property.
        let parameters = ("org.freedesktop.Secret.Collection", "Locked").to_variant();
        let value = self
            .collection
            .call_future(
                "org.freedesktop.DBus.Properties.Get",
                Some(&parameters),
                gio::DBusCallFlags::NO_AUTO_START,
                30_000,
            )
            .await
            .map_err(provider_error)?;
        value
            .get::<(glib::Variant,)>()
            .and_then(|(value,)| value.get::<bool>())
            .ok_or(BrokerError::BackendUnavailable)
    }

    pub(super) async fn unlocked(&self) -> Result<()> {
        if self.locked().await? {
            return Err(BrokerError::BackendDenied);
        }
        Ok(())
    }

    pub(super) async fn item(&self, reference: &SecretRef) -> Result<Option<Item>> {
        let attributes = attributes(reference);
        let lookup = attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let mut items = self
            .collection
            .search_future(None, lookup, SearchFlags::ALL)
            .await
            .map_err(provider_error)?;
        if items.len() > 1 {
            return Err(BrokerError::AccessDenied);
        }
        Ok(items.pop())
    }

    pub(super) async fn read(
        &self,
        reference: &SecretRef,
        revision: Option<&str>,
    ) -> Result<CredentialEntry> {
        self.unlocked().await?;
        let item = self.item(reference).await?.ok_or(BrokerError::NotFound)?;
        if item.is_locked() {
            return Err(BrokerError::BackendDenied);
        }
        item.load_secret_future().await.map_err(provider_error)?;
        let secret = item.secret().ok_or(BrokerError::BackendUnavailable)?;
        let bytes = Zeroizing::new(secret.get());
        if bytes.len() > 256 * 1024 {
            return Err(BrokerError::InvalidEncoding);
        }
        let payload = std::str::from_utf8(&bytes).map_err(|_| BrokerError::InvalidEncoding)?;
        custody_codec::decode(reference, revision, payload)
    }
}
