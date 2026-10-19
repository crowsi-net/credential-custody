use super::*;

#[test]
fn broken_worker_is_permanently_unavailable_without_replay() {
    let (sender, receiver) = mpsc::sync_channel(1);
    drop(receiver);
    let mut transport = Transport {
        sender: Some(sender),
    };
    assert!(matches!(
        transport.call(Operation::Lock),
        Err(BrokerError::BackendUnavailable)
    ));
    assert!(transport.sender.is_none());
    assert!(matches!(
        transport.call(Operation::Lock),
        Err(BrokerError::BackendUnavailable)
    ));
}

#[test]
fn uncertain_worker_response_invalidates_the_connection() {
    let (sender, receiver) = mpsc::sync_channel::<Message>(1);
    let worker = thread::spawn(move || {
        let (_, reply) = receiver.recv().expect("first request");
        reply.send(Err(BrokerError::BackendUnavailable)).ok();
        assert!(
            receiver.recv().is_err(),
            "there must be no automatic replay"
        );
    });
    let mut transport = Transport {
        sender: Some(sender),
    };
    assert!(matches!(
        transport.call(Operation::Lock),
        Err(BrokerError::BackendUnavailable)
    ));
    assert!(transport.sender.is_none());
    worker.join().expect("worker");
}

#[test]
fn denied_response_does_not_turn_into_automatic_unlock() {
    let (sender, receiver) = mpsc::sync_channel::<Message>(1);
    let worker = thread::spawn(move || {
        let (_, reply) = receiver.recv().expect("first request");
        reply.send(Err(BrokerError::BackendDenied)).ok();
        let (next, reply) = receiver.recv().expect("explicit status request");
        assert!(matches!(next, Operation::IsLocked));
        reply.send(Ok(Response::Locked(true))).ok();
    });
    let mut transport = Transport {
        sender: Some(sender),
    };
    assert!(matches!(
        transport.call(Operation::Lock),
        Err(BrokerError::BackendDenied)
    ));
    assert!(matches!(
        transport.call(Operation::IsLocked),
        Ok(Response::Locked(true))
    ));
    worker.join().expect("worker");
}
