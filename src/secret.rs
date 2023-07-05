use aes_gcm::{
    aead::{
        Aead, AeadCore, KeyInit, OsRng,
        generic_array::GenericArray,
    },
    Aes256Gcm,
};
use hex;
use rand::Rng;

pub const AES256_KEY_LEN: usize = 32;
pub const AES256_NONCE_LEN: usize = 12;

pub fn encrypt(
    plaintext: &str,
    key: &[u8; AES256_KEY_LEN],
    nonce: &[u8; AES256_NONCE_LEN]
) -> aes_gcm::aead::Result<String> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = GenericArray::from_slice(nonce);
    let aes_encrypted = cipher.encrypt(&nonce, plaintext.as_ref())?;
    let hex_encoded = hex::encode(aes_encrypted);

    Ok(hex_encoded)
}

pub fn decrypt(
    ciphertext: &str,
    key: &[u8; AES256_KEY_LEN],
    nonce: &[u8; AES256_NONCE_LEN]
) -> aes_gcm::aead::Result<String> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = GenericArray::from_slice(nonce);
    let hex_decoded = hex::decode(ciphertext).unwrap();
    let aes_decrypted = cipher.decrypt(&nonce, hex_decoded.as_ref())?;
    let plaintext = std::str::from_utf8(&aes_decrypted).unwrap().to_string();

    Ok(plaintext)
}

pub fn generate_key() -> [u8; AES256_KEY_LEN] {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(AES256_KEY_LEN)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

pub fn generate_nonce() -> [u8; AES256_NONCE_LEN] {
    Aes256Gcm::generate_nonce(&mut OsRng).into()
}

