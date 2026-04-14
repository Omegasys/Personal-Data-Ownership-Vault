use data_vault::crypto::key_manager::KeyManager;

fn main() {
    println!("🔑 Generating new Vault master key...");

    let km = KeyManager::new();
    let key = km.get_master_key();

    println!("✔ Key generated:");
    println!("{:02x?}", key);

    println!("⚠️ Store this securely. It is NOT recoverable.");
}
