use data_vault::storage::local_store::LocalStore;

#[test]
fn test_local_store_put_get() {
    let store = LocalStore::new("/tmp/vault_test_store");

    let key = "test_key";
    let value = b"test_value";

    store.put(key, value);
    let result = store.get(key).unwrap();

    assert_eq!(value.to_vec(), result);
}

#[test]
fn test_local_store_delete() {
    let store = LocalStore::new("/tmp/vault_test_store");

    let key = "delete_key";
    store.put(key, b"data");

    store.delete(key);

    assert!(store.get(key).is_none());
}
