use std::{
    io::Write,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};
pub fn phrase() -> String {
    format!("{} art", vec!["abandon"; 23].join(" "))
}
pub fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_crowsi-owner-recovery-provider"))
        .args(args)
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("synthetic provider process");
    let _ = child
        .stdin
        .take()
        .expect("protected stdin")
        .write_all(input);
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait().expect("provider status").is_none() {
        if Instant::now() >= deadline {
            child.kill().expect("stop owned child");
            child.wait().expect("reap owned child");
            panic!("synthetic provider exceeded deadline");
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().expect("provider result");
    assert!(output.stdout.len() <= 4096 && output.stderr.len() <= 1024);
    assert!(!String::from_utf8_lossy(&output.stdout).contains(&phrase()));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(&phrase()));
    output
}
