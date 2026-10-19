use super::{
    backend::{Backend, provider_error},
    transport::{Operation, Response},
};
use crate::{BrokerError, Result};

impl Backend {
    pub(super) async fn execute(&self, operation: Operation) -> Result<Response> {
        match operation {
            Operation::IsLocked => Ok(Response::Locked(
                self.0.is_locked().await.map_err(provider_error)?,
            )),
            Operation::Lock => {
                self.0.lock(None).await.map_err(provider_error)?;
                if !self.0.is_locked().await.map_err(provider_error)? {
                    return Err(BrokerError::BackendDenied);
                }
                Ok(Response::Done)
            }
            Operation::Unlock => {
                self.0.unlock(None).await.map_err(provider_error)?;
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
                self.unlocked().await?;
                self.item(&reference)
                    .await?
                    .ok_or(BrokerError::NotFound)?
                    .delete(None)
                    .await
                    .map_err(provider_error)?;
                if self.item(&reference).await?.is_some() {
                    return Err(BrokerError::BackendUnavailable);
                }
                Ok(Response::Done)
            }
        }
    }
}
