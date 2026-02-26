use sha2::{Digest, Sha256};

use crate::model::TokenType;

const BODY_HALF_LEN: usize = 22; // base62(u128::MAX) = 22 chars
const BODY_LEN: usize = BODY_HALF_LEN * 2; // two u128 halves = 44 chars
const CHECKSUM_LEN: usize = 6; // base62(u32::MAX) = 6 chars

fn base62_encode_padded(value: u128, width: usize) -> String {
    let encoded = base62::encode(value);
    format!("{:0>width$}", encoded, width = width)
}

pub fn generate_access_token(token_type: &TokenType) -> String {
    use rand::RngExt as _;
    let mut rng = rand::rng();
    let bytes: [u8; 32] = rng.random();

    let prefix = token_type.prefix();
    let hi = u128::from_be_bytes(bytes[..16].try_into().unwrap());
    let lo = u128::from_be_bytes(bytes[16..].try_into().unwrap());
    let body = format!(
        "{}{}",
        base62_encode_padded(hi, BODY_HALF_LEN),
        base62_encode_padded(lo, BODY_HALF_LEN)
    );
    let crc = crc32fast::hash(&bytes);
    let checksum = base62_encode_padded(crc as u128, CHECKSUM_LEN);

    format!("{prefix}{body}{checksum}")
}

pub fn validate_token_format(token: &str) -> bool {
    let rest = [TokenType::Personal, TokenType::Runner]
        .iter()
        .find_map(|t| token.strip_prefix(t.prefix()));
    let Some(rest) = rest else {
        return false;
    };
    if rest.len() != BODY_LEN + CHECKSUM_LEN {
        return false;
    }

    let (body, checksum_str) = rest.split_at(BODY_LEN);
    let (hi_str, lo_str) = body.split_at(BODY_HALF_LEN);

    let Ok(hi) = base62::decode(hi_str) else {
        return false;
    };
    let Ok(lo) = base62::decode(lo_str) else {
        return false;
    };
    let Ok(crc_val) = base62::decode(checksum_str) else {
        return false;
    };

    let hi_bytes = (hi as u128).to_be_bytes();
    let lo_bytes = (lo as u128).to_be_bytes();

    let mut body_bytes = [0u8; 32];
    body_bytes[..16].copy_from_slice(&hi_bytes);
    body_bytes[16..].copy_from_slice(&lo_bytes);

    let expected_crc = crc32fast::hash(&body_bytes);
    expected_crc as u128 == crc_val
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_personal_token_has_correct_format() {
        let token = generate_access_token(&TokenType::Personal);
        assert!(token.starts_with("gdp_"));
        assert_eq!(token.len(), 54);
        assert!(validate_token_format(&token));
    }

    #[test]
    fn test_generated_runner_token_has_correct_format() {
        let token = generate_access_token(&TokenType::Runner);
        assert!(token.starts_with("gdr_"));
        assert_eq!(token.len(), 54);
        assert!(validate_token_format(&token));
    }

    #[test]
    fn test_token_is_alphanumeric() {
        let token = generate_access_token(&TokenType::Personal);
        let body = &token[4..]; // strip prefix
        assert!(body.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_validate_rejects_bad_prefix() {
        assert!(!validate_token_format(
            "bad_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABB"
        ));
    }

    #[test]
    fn test_validate_rejects_corrupted_checksum() {
        let mut token = generate_access_token(&TokenType::Personal);
        // Flip last character
        let last = token.pop().unwrap();
        let replacement = if last == 'A' { 'B' } else { 'A' };
        token.push(replacement);
        assert!(!validate_token_format(&token));
    }
}
