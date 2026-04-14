use std::fs;
use std::path::Path;

use rand::rngs::OsRng;
use rand::RngCore;

pub struct KeyManager {
    pub master_key: [u8; 32],
}

impl KeyManager {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self { master_key: key }
    }

    pub fn load_or_create(path: &str) -> Self {
        if Path::new(path).exists() {
            let data = fs::read(path).expect("Failed to read key file");
            let mut key = [0u8; 32];
            key.copy_from_slice(&data[..32]);
            Self { master_key: key }
        } else {
            let km = Self::new();
            fs::write(path, &km.master_key).expect("Failed to write key");
            km
        }
    }

    pub fn get_master_key(&self) -> &[u8; 32] {
        &self.master_key
    }
}
