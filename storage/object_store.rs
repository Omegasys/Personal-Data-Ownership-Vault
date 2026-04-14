use crate::crypto::hashing::Hashing;
use crate::storage::local_store::LocalStore;

pub struct ObjectStore {
    store: LocalStore,
    chunk_size: usize,
}

impl ObjectStore {
    pub fn new(store: LocalStore, chunk_size: usize) -> Self {
        Self { store, chunk_size }
    }

    pub fn put(&self, data: &[u8]) -> Vec<String> {
        let mut keys = Vec::new();

        for chunk in data.chunks(self.chunk_size) {
            let hash = Hashing::sha256_hex(chunk);
            self.store.put(&hash, chunk);
            keys.push(hash);
        }

        keys
    }

    pub fn get(&self, keys: &[String]) -> Vec<u8> {
        let mut result = Vec::new();

        for key in keys {
            if let Some(chunk) = self.store.get(key) {
                result.extend(chunk);
            }
        }

        result
    }
}
