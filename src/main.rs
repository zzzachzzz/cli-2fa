#![allow(dead_code, unused_imports)]
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Parser, Subcommand};
use data_encoding;

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
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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


fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code: Option<ExitCode> = match cli.command {
        Some(command) => {
            match command {
                Commands::Add { name, secret } => {
                    let name = name.unwrap_or_else(|| prompt_input("Enter name of 2FA entry:"));
                    let secret = secret.unwrap_or_else(|| prompt_input("Enter secret of 2FA entry:"));
                    // TODO Validate secret
                    println!("name: {}", name);
                    println!("secret: {}", secret);

                    if let Err(base32_decode_err) = data_encoding::BASE32.decode(secret.as_bytes()) {
                        eprintln!("Invalid 2FA secret. Base32 decoding failed with error:\n\"{}\"", &base32_decode_err);
                        return ExitCode::from(65); // EX_DATAERR
                    }

                    let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()
                        .expect("Unable to get keyring entry");

                    let mut storage_ = storage::read_from_file(&key, &nonce)
                        .expect("Unable to load storage file");

                    storage_.map.insert(name.clone(), secret);

                    storage::write_to_file(&storage_, &key, &nonce)
                        .expect("Unable to write to storage file");

                    println!("Added 2FA entry \"{}\"", &name);
                    None
                },
                Commands::List { .. } => {
                    let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()
                        .expect("Unable to get keyring entry");

                    let storage_ = storage::read_from_file(&key, &nonce)
                        .expect("Unable to load storage file");

                    for entry_name in storage_.map.keys() {
                        println!("{}", entry_name);
                    }
                    None
                },
                Commands::Code { name } => {
                    let (key, nonce) = keyring::get_keyring_entry_key_and_nonce()
                        .expect("Unable to get keyring entry");

                    let storage_ = storage::read_from_file(&key, &nonce)
                        .expect("Unable to load storage file");

                    let maybe_entry = storage_.map.get(&name);

                    if maybe_entry.is_none() {
                        eprintln!("No 2FA entry found for name \"{}\"", &name);
                        return ExitCode::from(65); // EX_DATAERR
                    }

                    let totp_secret = maybe_entry.unwrap();

                    let totp_ = totp::generate_totp(totp_secret.as_bytes());
                    println!("{}", &totp_);
                    None
                },
            }
        },
        None => {
            None
        },
    };

    return exit_code.unwrap_or(ExitCode::from(0));

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
}

