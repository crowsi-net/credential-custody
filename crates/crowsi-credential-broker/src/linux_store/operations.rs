use super::{
    backend::{Backend, attributes, provider_error},
    transport::{Operation, Response},
};
use crate::{BrokerError, CredentialEntry, Result, custody_codec};
use gio::prelude::*;
use libsecret::{Item, ItemCreateFlags, Value, prelude::*};

impl Backend {
    async fn put(&self, entry: CredentialEntry) -> Result<()> {
        self.unlocked().await?;
        let metadata = entry.metadata();
        let reference = metadata.reference();
        if self.item(reference).await?.is_some()
            && self.read(reference, None).await?.metadata().revision() == metadata.revision()
        {
            return Err(BrokerError::CredentialChanged);
        }
        let payload = custody_codec::encode(&entry)?;
        let value = Value::new(&payload, "application/json");
        let attributes = attributes(reference);
        let attributes = attributes
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str()))
            .collect();
        Item::create_future(
            &self.collection,
            None,
            attributes,
            "Crowsi credential",
            &value,
            ItemCreateFlags::REPLACE,
        )
        .await
        .map_err(provider_error)?;
        let read_back = self
            .read(reference, Some(metadata.revision().as_str()))
            .await
            .map_err(|_| BrokerError::BackendUnavailable)?;
        if *custody_codec::encode(&read_back)? != *payload {
            return Err(BrokerError::BackendUnavailable);
        }
        Ok(())
    }

    pub(super) async fn execute(&self, operation: Operation) -> Result<Response> {
        match operation {
            Operation::IsLocked => Ok(Response::Locked(self.locked().await?)),
            Operation::Lock => {
                self.service
                    .lock_dbus_paths_future(&[self.collection.object_path().as_str()])
                    .await
                    .map_err(provider_error)?;
                if !self.locked().await? {
                    return Err(BrokerError::BackendUnavailable);
                }
                Ok(Response::Done)
            }
            Operation::Unlock => {
                self.service
                    .unlock_dbus_paths_future(&[self.collection.object_path().as_str()])
                    .await
                    .map_err(provider_error)?;
                self.unlocked().await?;
                Ok(Response::Done)
            }
            Operation::Get(reference, revision) => Ok(Response::Entry(
                self.read(&reference, revision.as_deref()).await?,
            )),
            Operation::Put(entry) => {
                self.put(entry).await?;
                Ok(Response::Done)
            }
            Operation::Delete(reference) => {
                // Searchable attributes alone are not an authenticated binding.
                self.read(&reference, None).await?;
                self.item(&reference)
                    .await?
                    .ok_or(BrokerError::NotFound)?
                    .delete_future()
                    .await
                    .map_err(provider_error)?;
                if self
                    .item(&reference)
                    .await
                    .map_err(|_| BrokerError::BackendUnavailable)?
                    .is_some()
                {
                    return Err(BrokerError::BackendUnavailable);
                }
                Ok(Response::Done)
            }
        }
    }
}
