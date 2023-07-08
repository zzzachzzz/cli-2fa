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
const STORAGE_FILENAME_TMP: &str = "storage.json.enc.tmp";

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

// TODO Do better than unwraps to catch decryption errors
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
    let filepath = get_storage_filepath();

    _write_to_file(
        filepath,
        storage,
        key,
        nonce
    )
}

pub fn write_to_tmp_file(
    storage: &Storage,
    key: &[u8; secret::AES256_KEY_LEN],
    nonce: &[u8; secret::AES256_NONCE_LEN],
) -> std::io::Result<()> {
    let filepath = get_storage_filepath_tmp();

    _write_to_file(
        filepath,
        storage,
        key,
        nonce
    )
}

pub fn overwrite_main_file_with_tmp_file() -> std::io::Result<()> {
    let main_filepath = get_storage_filepath();
    let tmp_filepath = get_storage_filepath_tmp();
    std::fs::rename(tmp_filepath, main_filepath)?;

    Ok(())
}

fn _write_to_file(
    filepath: std::path::PathBuf,
    storage: &Storage,
    key: &[u8; secret::AES256_KEY_LEN],
    nonce: &[u8; secret::AES256_NONCE_LEN],
) -> std::io::Result<()> {
    let storage_json = serde_json::to_string(&storage).unwrap();
    let storage_json_enc = secret::encrypt(&storage_json, &key, &nonce).unwrap();

    std::fs::create_dir_all(filepath.parent().unwrap())?;
    // "Depending on the platform, this function may fail if the full directory path does not exist."
    std::fs::write(filepath, storage_json_enc)?;

    Ok(())
}

fn get_project_dir() -> ProjectDirs {
    ProjectDirs::from("", "", APP_NAME).unwrap()
}

pub fn get_storage_filepath() -> std::path::PathBuf {
    let project_dir = get_project_dir();
    let storage_filepath = project_dir.data_dir().join(STORAGE_FILENAME);

    storage_filepath
}

pub fn get_storage_filepath_tmp() -> std::path::PathBuf {
    let project_dir = get_project_dir();
    let storage_filepath = project_dir.data_dir().join(STORAGE_FILENAME_TMP);

    storage_filepath
}

