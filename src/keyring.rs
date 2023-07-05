use keyring as kr;
use crate::secret;

/// In the MacOS Keychain, this is called the 'Name'
const KEYRING_SERVICE: &str = "cli-2fa";
/// In the MacOS Keychain, this is called the 'Account'
// TODO Name something like 'totp_secret_encryption_password'
const KEYRING_USER: &str = "ummm_hello_333";

pub fn get_keyring_entry_password() -> kr::Result<String> {
    kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?.get_password()
}

pub fn set_keyring_entry() -> kr::Result<kr::Entry> {
    let entry = kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    let random_password_bytes: [u8; 32] = secret::generate_random_password();
    let random_password_str: &str = std::str::from_utf8(&random_password_bytes).unwrap();

    entry.set_password(random_password_str)?;
    Ok(entry)
}

