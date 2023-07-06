#![allow(dead_code, unused_imports)]
use std::path::PathBuf;
use clap::{Parser, Subcommand};

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
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();
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

    let (key, nonce) = keyring::get_keyring_entry_key_and_nonce().unwrap();

    let storage_ = storage::Storage {
        map: std::collections::HashMap::from([
            ("Einar".to_string(), "Norway".to_string()),
            ("foo".to_string(), "bar".to_string()),
        ]),
    };

    // storage::write_to_file(&storage_, &key, &nonce).unwrap();

    let storage_read = storage::read_from_file(&key, &nonce).unwrap();
    println!("storage_read: {:?}", storage_read);
}

