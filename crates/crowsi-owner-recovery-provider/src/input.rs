use crowsi_owner_recovery_provider::parse;
use std::io::{self, IsTerminal, Read};
use zeroize::Zeroizing;
const MAXIMUM_SECRET_BYTES: u64 = 1_024;

pub(crate) fn secret_from_stdin()
-> Result<crowsi_owner_recovery_provider::RecoveryMnemonic, &'static str> {
    if io::stdin().is_terminal() {
        return Err("protected-stdin-required");
    }
    let mut value = Zeroizing::new(String::new());
    io::stdin()
        .take(MAXIMUM_SECRET_BYTES + 1)
        .read_to_string(&mut value)
        .map_err(|_| "secret-read-failed")?;
    if value.len() as u64 > MAXIMUM_SECRET_BYTES {
        return Err("secret-input-too-large");
    }
    let trimmed = value.trim_end_matches(['\r', '\n']);
    parse(trimmed).map_err(|_| "mnemonic-invalid")
}

pub(crate) fn number(value: &str) -> Result<u64, &'static str> {
    value.parse().map_err(|_| "number-invalid")
}
