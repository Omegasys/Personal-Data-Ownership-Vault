use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;

pub struct Encryption;

impl Encryption {
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, [u8; 12]) {
        let cipher = ChaCha20Poly1305::new(key.into());

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .expect("encryption failure!");

        (ciphertext, nonce_bytes)
    }

    pub fn decrypt(key: &[u8; 32], ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Vec<u8> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext)
            .expect("decryption failure!")
    }
}
