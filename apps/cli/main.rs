use std::io::{self, Write};

use data_vault::core::vault::Vault;
use data_vault::core::identity::Identity;
use data_vault::core::audit::AuditLog;
use data_vault::core::permissions::PermissionManager;

fn main() {
    println!("🧠 Personal Data Vault CLI");
    println!("Type 'help' for commands");

    let identity = Identity::new();
    let mut vault = Vault::new(identity);

    loop {
        print!("vault> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["exit"] => break,

            ["help"] => {
                println!("Commands:");
                println!("  store <key> <value>");
                println!("  get <key>");
                println!("  delete <key>");
                println!("  exit");
            }

            ["store", key, value] => {
                vault.store_data(key, value.as_bytes());
                println!("✔ stored {}", key);
            }

            ["get", key] => {
                match vault.retrieve_data(key) {
                    Some(data) => println!("📦 {:?}", String::from_utf8_lossy(&data)),
                    None => println!("not found"),
                }
            }

            ["delete", key] => {
                vault.delete_data(key);
                println!("🗑 deleted {}", key);
            }

            _ => println!("unknown command"),
        }
    }
}
