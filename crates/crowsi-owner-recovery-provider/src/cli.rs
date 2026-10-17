use crate::{
    commands::{approve, create, verify},
    input::number,
    output::inspect_descriptor,
};

pub(crate) fn run(arguments: &[String]) -> Result<(), &'static str> {
    match arguments {
        [
            command,
            descriptor_path,
            recovery,
            subject,
            provider,
            revision,
            observed,
        ] if command == "create" => create(
            descriptor_path,
            recovery,
            subject,
            provider,
            number(revision)?,
            number(observed)?,
        ),
        [command, recovery, subject, provider, revision, observed] if command == "verify-stdin" => {
            verify(
                recovery,
                subject,
                provider,
                number(revision)?,
                number(observed)?,
            )
        }
        [command, approval, binding, key, issued, expires] if command == "approve-stdin" => {
            approve(approval, binding, key, number(issued)?, number(expires)?)
        }
        [command, path] if command == "inspect-descriptor" => inspect_descriptor(path),
        _ => Err("usage-invalid"),
    }
}
