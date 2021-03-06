use solana_runtime::accounts_db::AccountStorageEntry;
use solana_runtime::status_cache::SlotDelta;
use solana_sdk::{clock::Slot, transaction};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::Arc;
use tempfile::TempDir;

pub type SnapshotPackageSender = Sender<SnapshotPackage>;
pub type SnapshotPackageReceiver = Receiver<SnapshotPackage>;
pub type SnapshotPackageSendError = SendError<SnapshotPackage>;

#[derive(Debug)]
pub struct SnapshotPackage {
    pub root: Slot,
    pub slot_deltas: Vec<SlotDelta<transaction::Result<()>>>,
    pub snapshot_links: TempDir,
    pub storage_entries: Vec<Arc<AccountStorageEntry>>,
    pub tar_output_file: PathBuf,
}

impl SnapshotPackage {
    pub fn new(
        root: Slot,
        slot_deltas: Vec<SlotDelta<transaction::Result<()>>>,
        snapshot_links: TempDir,
        storage_entries: Vec<Arc<AccountStorageEntry>>,
        tar_output_file: PathBuf,
    ) -> Self {
        Self {
            root,
            slot_deltas,
            snapshot_links,
            storage_entries,
            tar_output_file,
        }
    }
}
