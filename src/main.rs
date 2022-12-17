mod totp;

fn main() {
    let secret: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====";
    let otp = totp::generate_totp(&secret);
    println!("{}", &otp);
}

