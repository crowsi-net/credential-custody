use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use crate::{AgentError, replay::DurableLedger};
#[path = "../tests/support/private_directory.rs"]
mod private_directory;

pub(super) struct Fixture {
    pub state: PathBuf,
    pub anchor: PathBuf,
    uid: u32,
    _directory: tempfile::TempDir,
    _isolation: MutexGuard<'static, ()>,
}

static FIXTURE_ISOLATION: Mutex<()> = Mutex::new(());

impl Fixture {
    pub fn new() -> Self {
        let value = Self::new_uninitialized();
        value.initialize("test-ledger", 1).expect("initialize");
        value
    }
    pub fn new_for(ledger: &str) -> Self {
        let value = Self::new_uninitialized();
        value.initialize(ledger, 1).expect("initialize");
        value
    }
    pub fn new_uninitialized() -> Self {
        let isolation = FIXTURE_ISOLATION
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let directory = private_directory::create("ledger");
        let root = directory.path().to_path_buf();
        let state = root.join("state");
        let anchor = root.join("anchor");
        fs::create_dir_all(&state).expect("state");
        fs::create_dir(&anchor).expect("anchor");
        for path in [&root, &state, &anchor] {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("mode");
        }
        let uid = fs::metadata(&state).expect("uid").uid();
        Self {
            state,
            anchor,
            uid,
            _directory: directory,
            _isolation: isolation,
        }
    }
    pub fn initialize(&self, ledger: &str, generation: u64) -> Result<(), AgentError> {
        DurableLedger::initialize_once(
            &self.state,
            &self.anchor,
            self.uid,
            "endpoint-a",
            generation,
            ledger,
        )
    }
    pub fn ledger(&self, generation: u64) -> DurableLedger {
        self.try_ledger("endpoint-a", generation).expect("ledger")
    }
    pub fn try_ledger(
        &self,
        deployment: &str,
        generation: u64,
    ) -> Result<DurableLedger, AgentError> {
        DurableLedger::open(
            &self.state,
            &self.anchor,
            self.uid,
            deployment,
            generation,
            "test-ledger",
        )
    }
}

pub(super) fn matching(root: &Path, prefix: &str) -> usize {
    fs::read_dir(root)
        .expect("entries")
        .filter(|entry| {
            entry
                .as_ref()
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .starts_with(prefix)
        })
        .count()
}
