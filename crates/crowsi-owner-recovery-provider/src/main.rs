use std::process::ExitCode;
mod cli;
#[cfg(test)]
mod cli_tests;
mod commands;
mod input;
mod output;

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match cli::run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("crowsi-owner-recovery-provider: {reason}");
            ExitCode::FAILURE
        }
    }
}
