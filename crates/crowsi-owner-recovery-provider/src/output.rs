use crowsi_owner_recovery_provider::{RecoveryRootDescriptorV1, validate_descriptor};
use serde::Serialize;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub(crate) fn output(value: &impl Serialize) -> Result<(), &'static str> {
    serde_json::to_writer(io::stdout().lock(), value).map_err(|_| "output-failed")?;
    println!();
    Ok(())
}

pub(crate) fn write_descriptor(path: &str, value: &impl Serialize) -> Result<(), &'static str> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(path).map_err(|_| "descriptor-create-failed")?;
    serde_json::to_writer(&mut file, value).map_err(|_| "descriptor-write-failed")?;
    file.write_all(b"\n")
        .map_err(|_| "descriptor-write-failed")?;
    file.sync_all().map_err(|_| "descriptor-sync-failed")
}

pub(crate) fn inspect_descriptor(path: &str) -> Result<(), &'static str> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| "descriptor-unavailable")?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 16_384 {
        return Err("descriptor-invalid");
    }
    let file = std::fs::File::open(path).map_err(|_| "descriptor-unavailable")?;
    let value: RecoveryRootDescriptorV1 =
        serde_json::from_reader(file).map_err(|_| "descriptor-invalid")?;
    validate_descriptor(&value).map_err(|_| "descriptor-invalid")?;
    output(&value)
}
