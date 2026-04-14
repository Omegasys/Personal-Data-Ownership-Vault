use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer};

pub struct Identity {
    keypair: Keypair,
}

impl Identity {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        Self { keypair }
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).to_bytes().to_vec()
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}
