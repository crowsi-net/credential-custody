use std::{sync::mpsc, thread, time::Duration};

use crate::{BrokerError, CredentialEntry, Result, SecretRef};

use super::backend::Backend;

const DEADLINE: Duration = Duration::from_secs(40);

pub(super) enum Operation {
    Put(CredentialEntry),
    Get(SecretRef, Option<String>),
    Delete(SecretRef),
    IsLocked,
    Unlock,
    Lock,
}

pub(super) enum Response {
    Done,
    Locked(bool),
    Entry(CredentialEntry),
}

type Message = (Operation, mpsc::SyncSender<Result<Response>>);

pub(super) struct Transport {
    sender: Option<mpsc::SyncSender<Message>>,
}

impl Transport {
    pub(super) fn connect(alias: &str) -> Result<Self> {
        if alias.is_empty()
            || alias.len() > 64
            || !alias
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || alias == "session"
        {
            return Err(BrokerError::InvalidField("collection_alias"));
        }
        let alias = alias.to_owned();
        let (sender, receiver) = mpsc::sync_channel::<Message>(1);
        let (ready, startup) = mpsc::sync_channel(1);
        thread::Builder::new()
            .name("crowsi-secret-service".to_owned())
            .spawn(move || {
                let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                else {
                    let _ = ready.send(Err(BrokerError::BackendUnavailable));
                    return;
                };
                let backend = match runtime.block_on(bounded(Backend::connect(&alias))) {
                    Ok(backend) => backend,
                    Err(error) => {
                        let _ = ready.send(Err(error));
                        return;
                    }
                };
                if ready.send(Ok(())).is_err() {
                    return;
                }
                while let Ok((operation, reply)) = receiver.recv() {
                    let result = runtime.block_on(bounded(backend.execute(operation)));
                    let unavailable = matches!(result, Err(BrokerError::BackendUnavailable));
                    if reply.send(result).is_err() || unavailable {
                        break;
                    }
                }
            })
            .map_err(|_| BrokerError::BackendUnavailable)?;
        startup
            .recv_timeout(DEADLINE)
            .map_err(|_| BrokerError::BackendUnavailable)??;
        Ok(Self {
            sender: Some(sender),
        })
    }

    pub(super) fn call(&mut self, operation: Operation) -> Result<Response> {
        let sender = self
            .sender
            .as_ref()
            .ok_or(BrokerError::BackendUnavailable)?;
        let (reply, receive) = mpsc::sync_channel(1);
        let result = sender
            .send((operation, reply))
            .map_err(|_| BrokerError::BackendUnavailable)
            .and_then(|()| {
                receive
                    .recv_timeout(DEADLINE)
                    .map_err(|_| BrokerError::BackendUnavailable)
            })
            .and_then(|result| result);
        if matches!(result, Err(BrokerError::BackendUnavailable)) {
            self.sender.take();
        }
        result
    }
}

async fn bounded<T>(operation: impl std::future::Future<Output = Result<T>>) -> Result<T> {
    tokio::time::timeout(Duration::from_secs(35), operation)
        .await
        .map_err(|_| BrokerError::BackendUnavailable)?
}

#[cfg(test)]
#[path = "transport_tests.rs"]
mod tests;
