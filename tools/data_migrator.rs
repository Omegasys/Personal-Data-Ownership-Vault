use data_vault::storage::local_store::LocalStore;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: data_migrator <src_dir> <dst_dir>");
        return;
    }

    let src = &args[1];
    let dst = &args[2];

    println!("📦 Migrating data from {} → {}", src, dst);

    let src_store = LocalStore::new(src);
    let dst_store = LocalStore::new(dst);

    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let data = src_store.get(name);

                if let Some(bytes) = data {
                    dst_store.put(name, &bytes);
                    println!("✔ migrated {}", name);
                }
            }
        }
    }

    println!("✅ Migration complete");
}
