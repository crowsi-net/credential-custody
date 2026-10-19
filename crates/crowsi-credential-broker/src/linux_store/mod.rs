//! Explicit Secret Service custody; no fallback or local encryption implementation.
mod backend;
mod operations;
mod transport;
mod worker;

use std::sync::Mutex;

use crate::{CredentialEntry, CredentialMetadata, CredentialStore, Result, SecretRef};
use transport::{Operation, Response, Transport};

/// Uses an existing, explicitly selected OS keyring collection.
///
/// Unlock prompts belong to the provider. Locking affects the selected collection,
/// including other applications if a shared collection was selected. The connection
/// must be reopened after an unavailable/uncertain operation; writes are never retried.
/// This is software-backed custody, not hardware-nonexportable storage.
pub struct LinuxCustodyStore {
    transport: Mutex<Transport>,
}

impl LinuxCustodyStore {
    /// Connect without creating a collection or changing its lock state.
    pub fn connect(collection_alias: &str) -> Result<Self> {
        Ok(Self {
            transport: Mutex::new(Transport::connect(collection_alias)?),
        })
    }

    fn call(&self, operation: Operation) -> Result<Response> {
        self.transport
            .lock()
            .map_err(|_| crate::BrokerError::BackendUnavailable)?
            .call(operation)
    }

    /// Read provider state without retrieving any credential value.
    pub fn is_locked(&self) -> Result<bool> {
        match self.call(Operation::IsLocked)? {
            Response::Locked(value) => Ok(value),
            _ => Err(crate::BrokerError::BackendUnavailable),
        }
    }

    /// Ask the provider to unlock; never accepts or stores its master password.
    pub fn unlock(&self) -> Result<()> {
        self.call(Operation::Unlock).map(|_| ())
    }

    /// Explicitly lock the selected collection, not just this Rust handle.
    pub fn lock(&self) -> Result<()> {
        self.call(Operation::Lock).map(|_| ())
    }

    fn entry(&self, reference: &SecretRef, revision: Option<&str>) -> Result<CredentialEntry> {
        match self.call(Operation::Get(
            reference.clone(),
            revision.map(str::to_owned),
        ))? {
            Response::Entry(entry) => Ok(entry),
            _ => Err(crate::BrokerError::BackendUnavailable),
        }
    }
}

impl CredentialStore for LinuxCustodyStore {
    fn put(&self, entry: CredentialEntry) -> Result<()> {
        self.call(Operation::Put(entry)).map(|_| ())
    }

    fn metadata(&self, reference: &SecretRef) -> Result<CredentialMetadata> {
        Ok(self.entry(reference, None)?.metadata())
    }

    fn get(&self, reference: &SecretRef, expected_revision: &str) -> Result<CredentialEntry> {
        self.entry(reference, Some(expected_revision))
    }

    fn delete(&self, reference: &SecretRef) -> Result<()> {
        self.call(Operation::Delete(reference.clone())).map(|_| ())
    }
}
