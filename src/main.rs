#![allow(dead_code, unused_imports)]
mod totp;
mod ui;

fn main() {
    // let secret: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====";
    // let otp = totp::generate_totp(&secret);
    // println!("{}", &otp);

    _ = ui::example4();
    // _ = ui::instantfn();
}

