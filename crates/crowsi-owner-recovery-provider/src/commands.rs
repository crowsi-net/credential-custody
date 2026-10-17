use crate::{
    input::secret_from_stdin,
    output::{output, write_descriptor},
};
use crowsi_credential_authority_contracts::OwnerRecoveryCustodyStateV1;
use crowsi_owner_recovery_provider::{
    RecoveryApprovalInputV1, custody_receipt, descriptor, generate, issue_approval,
};
use std::io::{self, IsTerminal};

pub(crate) fn create(
    descriptor_path: &str,
    recovery: &str,
    subject: &str,
    provider: &str,
    revision: u64,
    observed: u64,
) -> Result<(), &'static str> {
    if !io::stderr().is_terminal() {
        return Err("owner-display-terminal-required");
    }
    let generated = generate().map_err(|_| "entropy-unavailable")?;
    let descriptor = descriptor(&generated.mnemonic);
    let receipt = custody_receipt(
        &generated.mnemonic,
        recovery,
        subject,
        provider,
        revision,
        OwnerRecoveryCustodyStateV1::Provisioned,
        observed,
    )
    .map_err(|_| "request-invalid")?;
    write_descriptor(descriptor_path, &descriptor)?;
    eprintln!("Owner recovery phrase (write it down offline; it is shown once):");
    eprintln!("{}", generated.mnemonic.expose_for_owner_ceremony());
    output(&receipt)
}

pub(crate) fn verify(
    recovery: &str,
    subject: &str,
    provider: &str,
    revision: u64,
    observed: u64,
) -> Result<(), &'static str> {
    let mnemonic = secret_from_stdin()?;
    let receipt = custody_receipt(
        &mnemonic,
        recovery,
        subject,
        provider,
        revision,
        OwnerRecoveryCustodyStateV1::Verified,
        observed,
    )
    .map_err(|_| "request-invalid")?;
    output(&receipt)
}

pub(crate) fn approve(
    approval_id: &str,
    binding_sha256: &str,
    key_id: &str,
    issued_at_epoch_s: u64,
    expires_at_epoch_s: u64,
) -> Result<(), &'static str> {
    let mnemonic = secret_from_stdin()?;
    let evidence = issue_approval(
        &mnemonic,
        &RecoveryApprovalInputV1 {
            approval_id: approval_id.into(),
            binding_sha256: binding_sha256.into(),
            key_id: key_id.into(),
            issued_at_epoch_s,
            expires_at_epoch_s,
        },
    )
    .map_err(|_| "approval-invalid")?;
    output(&evidence)
}
