#[path = "fixture/signing.rs"]
mod signing;
pub(crate) use signing::{digest, key, public, sign};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) struct Fixture {
    _isolation: MutexGuard<'static, ()>,
    _directory: tempfile::TempDir,
    pub(crate) root: PathBuf,
    pub(crate) store: PathBuf,
    pub(crate) authority_anchor: PathBuf,
    pub(crate) operations: PathBuf,
    pub(crate) management_state: PathBuf,
    pub(crate) provider_state: PathBuf,
    pub(crate) anchors: PathBuf,
    pub(crate) peer_state: PathBuf,
    pub(crate) peer_anchor: PathBuf,
    pub(crate) replay: PathBuf,
    pub(crate) replay_anchor: PathBuf,
    pub(crate) now: u64,
}

// Each case verifies finite process deadlines; parallel binary hashing would
// compete with those deadlines rather than test independent authority stores.
static PROCESS_CASE: Mutex<()> = Mutex::new(());

impl Fixture {
    pub(crate) fn new() -> Self {
        let isolation = PROCESS_CASE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // Private ancestry is part of the production trust boundary: /tmp is unsuitable.
        let private_directory = tempfile::Builder::new()
            .prefix(".authority-host-")
            .tempdir_in(std::env::current_dir().expect("current directory"))
            .expect("owner-private fixture");
        let root = private_directory.path().to_path_buf();
        let value = Self {
            _isolation: isolation,
            _directory: private_directory,
            store: root.join("store"),
            authority_anchor: root.join("authority-anchor"),
            operations: root.join("operations"),
            management_state: root.join("management-state"),
            provider_state: root.join("provider-state"),
            anchors: root.join("anchors"),
            peer_state: root.join("peer-state"),
            peer_anchor: root.join("peer-anchor"),
            replay: root.join("replay"),
            replay_anchor: root.join("replay-anchor"),
            root,
            now: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_secs(),
        };
        for path in [
            &value.store,
            &value.authority_anchor,
            &value.operations,
            &value.management_state,
            &value.provider_state,
            &value.anchors,
            &value.peer_state,
            &value.peer_anchor,
            &value.replay,
            &value.replay_anchor,
        ] {
            directory(path);
        }
        value
    }

    pub(crate) fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    pub(crate) fn write(&self, name: &str, wire: &[u8]) -> PathBuf {
        let path = self.path(name);
        fs::write(&path, wire).expect("write");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).expect("mode");
        path
    }
}

fn directory(path: &Path) {
    fs::create_dir(path).expect("directory");
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("mode");
}
