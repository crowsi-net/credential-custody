use super::Installation;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

impl Installation {
    pub fn run(&self, operation: &str) -> std::process::Output {
        let host_before = host_trust_metadata();
        let user = fs::read_link("/proc/self/ns/user").expect("isolated bootstrap setup");
        let mount = fs::read_link("/proc/self/ns/mnt").expect("isolated bootstrap setup");
        let mut child = Command::new("/usr/bin/unshare")
            .args([
                "--user",
                "--map-root-user",
                "--mount",
                "--net",
                "--pid",
                "--fork",
                "--kill-child=KILL",
                "/bin/sh",
            ])
            .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/production/namespace.sh"))
            .arg(&self.root)
            .arg(operation)
            .arg(user)
            .arg(mount)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("isolated bootstrap setup");
        if child
            .wait_timeout(Duration::from_secs(30))
            .expect("isolated bootstrap setup")
            .is_none()
        {
            child.kill().expect("isolated bootstrap setup");
            child.wait().expect("isolated bootstrap setup");
            panic!("isolated CLI timed out");
        }
        let output = child.wait_with_output().expect("isolated bootstrap setup");
        assert_eq!(host_before, host_trust_metadata(), "host trust changed");
        output
    }
}

fn host_trust_metadata() -> Option<(u64, u64, i64, u32, u32)> {
    use std::os::unix::fs::MetadataExt;
    fs::symlink_metadata("/etc/crowsi/device-credential-agent/root-trust.json")
        .ok()
        .map(|m| (m.ino(), m.len(), m.mtime(), m.uid(), m.mode()))
}
