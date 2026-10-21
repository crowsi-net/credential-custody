use std::{fs, path::PathBuf, time::{Duration, Instant}};
mod fixture;
use fixture::Fixture;

use crate::host_provider_process;

#[test]
fn provider_empty_stderr_is_valid_and_output_is_bounded() {
    let fixture = Fixture::new("finite");
    let output =
        host_provider_process::exchange(&fixture.route, "reissue", b"{}").expect("finite provider");
    assert_eq!(output, br#"{"ok":true}"#);
}

#[test]
fn writable_provider_executable_is_refused_before_spawn() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = Fixture::new("finite");
    fs::set_permissions(&fixture.route.executable, fs::Permissions::from_mode(0o770))
        .expect("writable executable");
    assert!(matches!(
        host_provider_process::exchange(&fixture.route, "reissue", b"{}"),
        Err(crate::HostError::PathInvalid)
    ));
    assert!(!fixture.pidfile.exists());
}

#[test]
fn provider_parent_success_kills_grandchild_without_pipe_hang() {
    let fixture = Fixture::new("grandchild-success");
    let started = Instant::now();
    assert_eq!(
        host_provider_process::exchange(&fixture.route, "reissue", b"{}").expect("response"),
        b"{}"
    );
    assert!(started.elapsed() < Duration::from_secs(3));
    let pid = fs::read_to_string(&fixture.pidfile).expect("grandchild pid");
    assert!(gone(pid.trim()));
}

#[test]
fn provider_timeout_kills_its_process_group() {
    let fixture = Fixture::new("grandchild-timeout");
    let started = Instant::now();
    assert!(host_provider_process::exchange(&fixture.route, "reissue", b"{}").is_err());
    assert!(started.elapsed() < Duration::from_secs(3));
    let pid = fs::read_to_string(&fixture.pidfile).expect("grandchild pid");
    assert!(gone(pid.trim()));
}

#[test]
fn provider_that_never_reads_large_stdin_is_bounded_and_killed() {
    let fixture = Fixture::new("blocked-input");
    let started = Instant::now();
    assert!(
        host_provider_process::exchange(&fixture.route, "reissue", &vec![b'x'; 2_097_152]).is_err()
    );
    assert!(started.elapsed() < Duration::from_secs(3));
    let pid = fs::read_to_string(&fixture.pidfile).expect("provider pid");
    assert!(gone(pid.trim()));
}

fn gone(pid: &str) -> bool {
    let process = PathBuf::from(format!("/proc/{pid}"));
    for _ in 0..50 {
        if !process.exists() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}
