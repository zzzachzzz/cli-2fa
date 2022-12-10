use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use data_encoding::BASE32;

type HmacSha1 = Hmac<Sha1>;

pub const INTERVAL: u64 = 30;

pub fn generate_totp(
    secret: &[u8]
) -> u32 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    _generate_totp(secret, INTERVAL, timestamp)
}

/// HOTP Algorithm: https://www.rfc-editor.org/rfc/rfc4226#section-5.3
pub fn _generate_totp(
    secret: &[u8],
    interval: u64,
    timestamp: u64
) -> u32 {
    let secret_decoded = BASE32.decode(secret)
        .expect(format!(
            "Secret could not be decoded with {}",
            stringify!(BASE32)
        ).as_str());

    // RFC Step 1: "Generate an HMAC-SHA-1 value"
    let mut hmac = HmacSha1::new_from_slice(&secret_decoded)
        .expect("HMAC can take a key of any size");
    hmac.update(&(timestamp / interval).to_be_bytes());
    let hmac_result = hmac.finalize().into_bytes();

    // RFC Step 2: "Generate a 4-byte string (Dynamic Truncation)"
    let offset = (hmac_result.last().unwrap() & 0xf) as usize;

    // Use 4 bytes starting from the `offset` to produce the dynamic binary code
    let mut bin_code = u32::from_be_bytes([
        hmac_result[offset],
        hmac_result[offset + 1],
        hmac_result[offset + 2],
        hmac_result[offset + 3]
    ]);

    // RFC Step 3: "Compute an HOTP value"
    // Use the last 31 bits of the code
    bin_code &= 0x7fffffff;

    // Get hotp based on number of desired digits
    let hotp = bin_code % u32::pow(10, 6);

    hotp
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use super::{_generate_totp, INTERVAL};

    #[test]
    fn works_for_padded_base32_secret() {
        let secret: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====";
        let timestamp = DateTime::parse_from_rfc3339("2022-12-18T16:43:53.737956458+00:00")
            .unwrap()
            .timestamp();

        let totp = _generate_totp(&secret, INTERVAL, timestamp as u64);

        assert_eq!(totp, 010210);
    }
}
