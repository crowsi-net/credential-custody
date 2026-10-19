use oo7::dbus::{Collection, Item, Service};
use sha2::{Digest, Sha256};

use crate::{BrokerError, CredentialEntry, Result, SecretRef, custody_codec};

pub(super) struct Backend(pub(super) Collection);

pub(super) fn provider_error(_: oo7::dbus::Error) -> BrokerError {
    // Do not propagate provider messages, object paths or secret-bearing details.
    BrokerError::BackendUnavailable
}

fn attributes(reference: &SecretRef) -> [(String, String); 2] {
    [
        ("application".into(), "crowsi-credential-broker-v1".into()),
        (
            "slot".into(),
            hex::encode(Sha256::digest(reference.canonical_id().as_bytes())),
        ),
    ]
}

impl Backend {
    pub(super) async fn connect(alias: &str) -> Result<Self> {
        // Explicit encrypted DBus transport; no automatic file/plain fallback.
        let service = Service::encrypted().await.map_err(provider_error)?;
        Ok(Self(
            service
                .with_alias(alias)
                .await
                .map_err(provider_error)?
                .ok_or(BrokerError::BackendUnavailable)?,
        ))
    }

    pub(super) async fn unlocked(&self) -> Result<()> {
        if self.0.is_locked().await.map_err(provider_error)? {
            return Err(BrokerError::BackendDenied);
        }
        Ok(())
    }

    pub(super) async fn item(&self, reference: &SecretRef) -> Result<Option<Item>> {
        let mut items = self
            .0
            .search_items(&attributes(reference))
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
        if item.is_locked().await.map_err(provider_error)? {
            return Err(BrokerError::BackendDenied);
        }
        let secret = item.secret().await.map_err(provider_error)?;
        if secret.as_bytes().len() > 256 * 1024 {
            return Err(BrokerError::InvalidEncoding);
        }
        let payload =
            std::str::from_utf8(secret.as_bytes()).map_err(|_| BrokerError::InvalidEncoding)?;
        custody_codec::decode(reference, revision, payload)
    }

    pub(super) async fn put(&self, entry: CredentialEntry) -> Result<()> {
        self.unlocked().await?;
        let metadata = entry.metadata();
        let reference = metadata.reference();
        if self.item(reference).await?.is_some()
            && self.read(reference, None).await?.metadata().revision() == metadata.revision()
        {
            return Err(BrokerError::CredentialChanged);
        }
        let payload = custody_codec::encode(&entry)?;
        self.0
            .create_item(
                "Crowsi credential",
                &attributes(reference),
                oo7::Secret::blob(payload.as_bytes()),
                true,
                None,
            )
            .await
            .map_err(provider_error)?;
        // Acknowledge only after reading back exactly what was written.
        let read_back = self
            .read(reference, Some(metadata.revision().as_str()))
            .await
            .map_err(|_| BrokerError::BackendUnavailable)?;
        if *custody_codec::encode(&read_back)? != *payload {
            return Err(BrokerError::BackendUnavailable);
        }
        Ok(())
    }
}
