#![allow(dead_code, unused_imports)]
use std::path::PathBuf;
use std::process::ExitCode;
use std::error::Error;
use clap::{Parser, Subcommand};
use data_encoding;
extern crate keyring as keyring_crate;
use keyring_crate::error::Error as KeyringCrateError;
use anyhow::Context;

mod totp;
mod ui;
mod chatgpt;
mod keyring;
mod secret;
mod storage;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new 2FA entry
    Add {
        /// Name of 2FA entry (will be prompted if omitted)
        #[arg(short, long)]
        name: Option<String>,

        /// Secret of 2FA entry (will be prompted if omitted)
        #[arg(short, long)]
        secret: Option<String>,
    },
    /// Lists stored 2FA entries
    List,
    /// Get a 2FA code for one or more entries
    Code {
        /// Name of 2FA entry
        #[arg(short, long)]
        name: String,
    },
}

fn prompt_input(prompt: &str) -> String {
    let prompt = || -> String {
        let mut input = String::new();
        println!("{}", &prompt);
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let cleaned_input = input.trim().to_string();
        cleaned_input
    };

    loop {
        let cleaned_input = prompt();
        if !cleaned_input.is_empty() { return cleaned_input; }
        println!("Input was empty or contained only whitespace. Try again.");
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add { name, secret }) => {
            let name = name.unwrap_or_else(|| prompt_input("Enter name of 2FA entry:"));
            let secret = secret.unwrap_or_else(|| prompt_input("Enter secret of 2FA entry:"));
            // TODO Validate secret
            println!("name: {}", name);
            println!("secret: {}", secret);

            // data_encoding::BASE32.decode(secret.as_bytes())
            //     .with_context(|| format!("Invalid 2FA secret. Base32 decoding failed."))?;

            let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()?;

            let mut storage_ = storage::read_from_file(&key, &nonce)
                .expect("Unable to load storage file");

            storage_.map.insert(name.clone(), secret);

            let new_nonce = secret::generate_nonce();

            // Write to temp file, encrypting with new nonce.
            // Do this so that if we fail to write the new nonce to the keyring,
            // that we are not left with an unreadable encrypted file, due to its
            // respective key & nonce not being written to the keyring.
            storage::write_to_tmp_file(&storage_, &key, &new_nonce)
                .with_context(|| format!(
                    "Unable to write to storage file at path \"{}\"",
                    storage::get_storage_filepath_tmp().display()
                ))?;

            keyring::set_keyring_entry(&key, &new_nonce)
                .with_context(|| format!(
                    "Failed to set keyring entry. Check the keyring entry for \"{}\".",
                    keyring::KEYRING_SERVICE
                ))?;

            // In the unlikely event that this fails, inform user that manual intervention is needed
            storage::overwrite_main_file_with_tmp_file()
                .with_context(|| format!("
Failed to move file \"{tmp_file}\", to path \"{main_file}\".
Manual intervention is needed!
The encryption key stored in the keyring
can only decrypt \"{tmp_file}\".
Try the following to ensure your 2FA entries are not lost:
1. Backup both of the files:
   - \"{tmp_file}\"
   - \"{main_file}\"
2. Manually attempt to move
   \"{tmp_file}\"
   to the location
   \"{main_file}\"
3. Check that everything is in order by running \"2fa list\"",
                    tmp_file = storage::get_storage_filepath_tmp().display(),
                    main_file = storage::get_storage_filepath().display()
                ))?;

            println!("Added 2FA entry \"{}\"", &name);
        },
        Some(Command::List { .. }) => {
            let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()
                .expect("Unable to get keyring entry");

            let storage_ = storage::read_from_file(&key, &nonce)
                .expect("Unable to load storage file");

            for entry_name in storage_.map.keys() {
                println!("{}", entry_name);
            }
        },
        Some(Command::Code { name }) => {
            let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()
                .expect("Unable to get keyring entry");

            let storage_ = storage::read_from_file(&key, &nonce)
                .expect("Unable to load storage file");

            let maybe_entry = storage_.map.get(&name);

            if maybe_entry.is_none() {
                eprintln!("No 2FA entry found for name \"{}\"", &name);
                // return ExitCode::from(65); // EX_DATAERR
            }

            let totp_secret = maybe_entry.unwrap();

            let totp_ = totp::generate_totp(totp_secret.as_bytes());
            println!("{}", &totp_);
        },
        None => {
            // TODO
        },
    }

    Ok(())
}

    // return match maybe_error {
    //     Some(err) => match {
    //     },
    //     None => ExitCode::SUCCESS,
    // };

    // let secret: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====";
    // let otp = totp::generate_totp(&secret);
    // println!("{}", &otp);

    // _ = ui::example1();
    // _ = ui::example2();
    // _ = ui::example3();
    // _ = ui::example4();
    // _ = ui::example5();
    // _ = ui::instantfn();
    // let res = keyring::get_keyring_entry_password();
    // let s = res.unwrap();
    // println!("s {}:", s);

    // let pass = secret::generate_random_password();
    // let random_password_str: &str = std::str::from_utf8(&pass).unwrap();
    // println!("random_password_str: {}", random_password_str);
    // let (encrypted, nonce) = secret::encrypt("foobar", &pass).unwrap();
    // println!("nonce: {:?}", nonce);
    // println!("encrypted: {}", encrypted);
    // let decrypted = secret::decrypt(encrypted.as_str(), &pass, &nonce).unwrap();
    // println!("decrypted: {}", decrypted);

    // let x = storage::load_from_file().unwrap();
    // println!("x: {:?}", x);

    // let x: [u8; 1] = [122];

    // let key = secret::generate_key();
    // let nonce = secret::generate_nonce();

//     let (key, nonce) = keyring::get_keyring_entry_key_and_nonce().unwrap();

//     let storage_ = storage::Storage {
//         map: std::collections::HashMap::from([
//             ("Einar".to_string(), "Norway".to_string()),
//             ("foo".to_string(), "bar".to_string()),
//         ]),
//     };

//     // storage::write_to_file(&storage_, &key, &nonce).unwrap();

//     let storage_read = storage::read_from_file(&key, &nonce).unwrap();
//     println!("storage_read: {:?}", storage_read);

