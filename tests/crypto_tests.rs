use data_vault::crypto::encryption::Encryption;
use data_vault::crypto::hashing::Hashing;

#[test]
fn test_encrypt_decrypt() {
    let key = Encryption::generate_key();
    let data = b"secret data";

    let (ciphertext, nonce) = Encryption::encrypt(&key, data);
    let decrypted = Encryption::decrypt(&key, &ciphertext, &nonce);

    assert_eq!(data.to_vec(), decrypted);
}

#[test]
fn test_hashing_consistency() {
    let data = b"hello world";

    let h1 = Hashing::sha256(data);
    let h2 = Hashing::sha256(data);

    assert_eq!(h1, h2);
}
