use keyring as kr;
use hex;
use crate::secret;

/// In the MacOS Keychain, this is called the 'Name'
pub const KEYRING_SERVICE: &str = "cli-2fa";
/// In the MacOS Keychain, this is called the 'Account'
pub const KEYRING_USER: &str = "totp-secret-encryption-key";

// TODO Use 'anyhow' here for other error types, such as 'FromHexError'?
pub fn get_keyring_entry_key_and_nonce() -> kr::Result<(
    [u8; secret::AES256_KEY_LEN],
    [u8; secret::AES256_NONCE_LEN],
)> {
    let keyring_entry_password = kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?.get_password()?;
    let bytes: [u8; secret::AES256_KEY_LEN + secret::AES256_NONCE_LEN] =
        hex::decode(keyring_entry_password)
        .expect("Unable to hex decode password from keyring")
        .try_into()
        .expect(&format!(
            "Unable to convert password from keyring to type [u8; {arr_len}]",
            arr_len = secret::AES256_KEY_LEN + secret::AES256_NONCE_LEN
        ));
    let key: [u8; secret::AES256_KEY_LEN] = bytes[0..secret::AES256_KEY_LEN].try_into().unwrap();
    let nonce: [u8; secret::AES256_NONCE_LEN] = bytes[secret::AES256_KEY_LEN..].try_into().unwrap();

    Ok((key, nonce))
}

pub fn set_keyring_entry(
    aes_key: &[u8; secret::AES256_KEY_LEN],
    aes_nonce: &[u8; secret::AES256_NONCE_LEN],
) -> kr::Result<kr::Entry> {
    let entry = kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;

    let mut combined: [u8; secret::AES256_KEY_LEN + secret::AES256_NONCE_LEN] =
        [0; secret::AES256_KEY_LEN + secret::AES256_NONCE_LEN];
    combined[..secret::AES256_KEY_LEN].copy_from_slice(aes_key);
    combined[secret::AES256_KEY_LEN..].copy_from_slice(aes_nonce);

    let keyring_entry_password: String = hex::encode(combined);
    entry.set_password(&keyring_entry_password)?;

    Ok(entry)
}

