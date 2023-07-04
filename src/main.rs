#![allow(dead_code, unused_imports)]
use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod totp;
mod ui;
mod chatgpt;
mod keyring;

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
    let res = keyring::get_keyring_entry_password();
    let s = res.unwrap();
    println!("s {}:", s);
    // let _ = chatgpt::the_main();
}

