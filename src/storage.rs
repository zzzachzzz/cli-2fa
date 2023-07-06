use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json;
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
    #[serde(flatten)]
    pub map: TotpMap,
}

pub fn read_from_file(
    key: &[u8; secret::AES256_KEY_LEN],
    nonce: &[u8; secret::AES256_NONCE_LEN],
) -> std::io::Result<Storage> {
    let storage_filepath = get_storage_filepath();
    let file_contents_enc = std::fs::read_to_string(storage_filepath)?;
    let file_contents_plain = secret::decrypt(&file_contents_enc, &key, &nonce).unwrap();
    let storage: Storage = serde_json::from_str(&file_contents_plain)?;

    Ok(storage)
}

pub fn write_to_file(
    storage: &Storage,
    key: &[u8; secret::AES256_KEY_LEN],
    nonce: &[u8; secret::AES256_NONCE_LEN],
) -> std::io::Result<()> {
    let storage_json = serde_json::to_string(&storage).unwrap();
    let storage_json_enc = secret::encrypt(&storage_json, &key, &nonce).unwrap();
    let storage_filepath = get_storage_filepath();

    std::fs::create_dir_all(storage_filepath.parent().unwrap())?;
    // "Depending on the platform, this function may fail if the full directory path does not exist."
    std::fs::write(storage_filepath, storage_json_enc)?;

    Ok(())
}

fn get_project_dir() -> ProjectDirs {
    ProjectDirs::from("", "", APP_NAME).unwrap()
}

fn get_storage_filepath() -> std::path::PathBuf {
    let project_dir = get_project_dir();
    let storage_filepath = project_dir.data_dir().join(STORAGE_FILENAME);

    storage_filepath
}

