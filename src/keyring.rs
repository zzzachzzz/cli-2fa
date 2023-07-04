use rand::Rng;
use keyring as kr;

/// In the MacOS Keychain, this is called the 'Name'
const KEYRING_SERVICE: &str = "cli-2fa";
/// In the MacOS Keychain, this is called the 'Account'
// TODO Name something like 'totp_secret_encryption_password'
const KEYRING_USER: &str = "ummm_hello_333";
const KEYRING_PASSWORD_LENGTH: usize = 30;

pub fn get_keyring_entry_password() -> kr::Result<String> {
    kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?.get_password()
}

pub fn set_keyring_entry() -> kr::Result<kr::Entry> {
    let entry = kr::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    let random_password = generate_random_password();
    entry.set_password(&random_password)?;
    Ok(entry)
}

fn generate_random_password() -> String {
    let random_password: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(KEYRING_PASSWORD_LENGTH)
        .map(char::from)
        .collect();

    random_password
}

