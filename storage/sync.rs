pub struct SyncManager;

impl SyncManager {
    pub fn new() -> Self {
        Self
    }

    pub fn sync_to_remote(&self) {
        // TODO: implement remote sync (IPFS, S3, P2P, etc.)
        println!("Sync to remote not implemented yet");
    }

    pub fn sync_from_remote(&self) {
        // TODO: implement pull from remote
        println!("Sync from remote not implemented yet");
    }
}
