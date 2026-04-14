use data_vault::storage::local_store::LocalStore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: vault_inspector <storage_path>");
        return;
    }

    let store = LocalStore::new(&args[1]);

    println!("🔍 Inspecting vault at: {}", args[1]);

    // NOTE: This assumes direct filesystem view (not encrypted awareness yet)
    println!("⚠️ Raw inspection mode (encrypted data will be unreadable)");
}
