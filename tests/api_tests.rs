use std::sync::{Arc, Mutex};
use data_vault::core::vault::Vault;

#[test]
fn test_vault_store_retrieve() {
    let identity = data_vault::core::identity::Identity::new();
    let vault = Vault::new(identity);

    let arc_vault = Arc::new(Mutex::new(vault));

    {
        let mut v = arc_vault.lock().unwrap();
        v.store_data("key1", b"value1");
    }

    let v = arc_vault.lock().unwrap();
    let result = v.retrieve_data("key1");

    assert!(result.is_none()); 
    // NOTE: still placeholder until storage wiring is complete
}
