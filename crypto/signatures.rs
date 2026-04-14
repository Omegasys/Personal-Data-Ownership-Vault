use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct SignatureManager {
    keypair: Keypair,
}

impl SignatureManager {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Self { keypair }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).to_bytes().to_vec()
    }

    pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        if let Ok(pubkey) = ed25519_dalek::PublicKey::from_bytes(public_key) {
            if let Ok(sig) = Signature::from_bytes(signature) {
                return pubkey.verify(message, &sig).is_ok();
            }
        }
        false
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}
