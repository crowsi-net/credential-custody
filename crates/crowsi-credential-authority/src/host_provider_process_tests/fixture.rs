use std::{fs, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, process::Command,
    sync::{Mutex, MutexGuard}};
use crate::host_config_types::HostProviderRoute;
static PROCESS_TESTS: Mutex<()> = Mutex::new(());

pub(super) struct Fixture {
    _serial: MutexGuard<'static, ()>,
    _root: tempfile::TempDir,
    pub(super) pidfile: PathBuf,
    pub(super) route: HostProviderRoute,
}

impl Fixture {
    pub(super) fn new(scenario: &str) -> Self {
        let serial = PROCESS_TESTS.lock().expect("provider process test lock");
        // Production trust rejects writable /tmp ancestors; retain private checkout ancestry.
        let root = tempfile::Builder::new().prefix("vpremises-provider-process-")
            .permissions(fs::Permissions::from_mode(0o700))
            .tempdir_in(std::env::current_dir().expect("current directory"))
            .expect("private provider fixture");
        let pidfile = root.path().join("child.pid");
        let executable = root.path().join("provider");
        let source = root.path().join("provider.rs");
        fs::write(&source, include_str!("native.rs")).expect("native provider fixture");
        let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
        assert!(Command::new(compiler).arg("--edition=2024").arg(&source)
            .arg("-o").arg(&executable).status().expect("compile provider").success());
        // Never inherit a permissive caller umask for an executable trusted by the host.
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).expect("executable mode");
        let config = root.path().join("provider.json");
        fs::write(&config, format!("{scenario}\n{}", pidfile.display())).expect("config");
        fs::set_permissions(&config, fs::Permissions::from_mode(0o600)).expect("config mode");
        let state = root.path().join("provider-state");
        fs::create_dir(&state).expect("state directory");
        fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).expect("state mode");
        Self {
            _serial: serial, _root: root, pidfile,
            route: HostProviderRoute {
                service_id: "service-a".into(), executable: text(&executable),
                executable_sha256: digest(&executable), config_path: text(&config),
                config_sha256: digest(&config), state_directory: text(&state),
                response_key_id: "provider-response".into(), response_public_key_hex: "11".repeat(32),
            },
        }
    }
}
fn digest(path: &Path) -> String {
    crate::host_crypto::digest(&fs::read(path).expect("fixture bytes"))
}
fn text(path: &Path) -> String { path.to_str().expect("utf8 path").to_owned() }
