use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

use std::fs::File;
use std::io::{Read, Write};

use directories::{BaseDirs, UserDirs, ProjectDirs};

use crate::keyring;
use crate::secret;

const APP_NAME: &str = "cli-2fa";
const STORAGE_FILENAME: &str = "storage.json.enc";

#[derive(Serialize, Deserialize)]
pub struct TotpEntry {
    name: String,
    secret: String,
}

pub type TotpName = String;
pub type TotpSecret = String;
pub type TotpMap = HashMap<TotpName, TotpSecret>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    totp_map: TotpMap,
}

pub fn load_from_file() -> serde_json::Result<Storage> {
    let json = r#"
        {
            "totp_map": {
                "key1": "value1",
                "key2": "value2",
                "key3": "value3"
            }
        }
    "#;

    let storage: Storage = serde_json::from_str(json)?;

    Ok(storage)
}



fn encrypt_file(key: &str, input_path: &str, output_path: &str) {
    // TODO get key & nonce
    let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()?;
    secret::decrypt("", &key, &nonce);

    let proj_dir = ProjectDirs::from("", "", APP_NAME).unwrap();
    let storage_file_path = proj_dir.data_dir().join(STORAGE_FILENAME);
    // Read the input file into a buffer
    let mut input_file = File::open(storage_file_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    // Initialize the cipher with the provided key
    let cipher = Aes256Cbc::new_varkey(key.as_bytes(), Default::default())?;

    // Encrypt the input data
    let ciphertext = cipher.encrypt_vec(&input_data);

    // Write the encrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}

