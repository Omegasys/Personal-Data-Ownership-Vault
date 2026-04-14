use sha2::{Sha256, Digest};

pub struct Hashing;

impl Hashing {
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn sha256_hex(data: &[u8]) -> String {
        let hash = Self::sha256(data);
        hash.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
