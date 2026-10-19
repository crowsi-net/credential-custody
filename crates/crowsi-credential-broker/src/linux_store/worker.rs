use super::{backend::Backend, transport::Message};
use crate::{BrokerError, Result};
use std::{sync::mpsc, time::Duration};

pub(super) fn run(
    alias: String,
    receiver: mpsc::Receiver<Message>,
    ready: mpsc::SyncSender<Result<()>>,
) {
    let context = glib::MainContext::new();
    let acquired = context.with_thread_default(|| {
        let backend = match context.block_on(bounded(Backend::connect(&alias))) {
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
            let result = context.block_on(bounded(backend.execute(operation)));
            let unavailable = matches!(result, Err(BrokerError::BackendUnavailable));
            if reply.send(result).is_err() || unavailable {
                break;
            }
        }
    });
    if acquired.is_err() {
        let _ = ready.send(Err(BrokerError::BackendUnavailable));
    }
}

async fn bounded<T>(operation: impl std::future::Future<Output = Result<T>>) -> Result<T> {
    glib::future_with_timeout(Duration::from_secs(35), operation)
        .await
        .map_err(|_| BrokerError::BackendUnavailable)?
}
