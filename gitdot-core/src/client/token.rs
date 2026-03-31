use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt as _;

use crate::{model::TokenType, util::crypto::hash_string};

const AUTH_CODE_EXPIRY_MINUTES: i64 = 10;
const DEVICE_CODE_EXPIRY_MINUTES: i64 = 10;
const POLLING_INTERVAL_SECONDS: u64 = 1;

const BODY_HALF_LEN: usize = 22; // base62(u128::MAX) = 22 chars
const BODY_LEN: usize = BODY_HALF_LEN * 2; // two u128 halves = 44 chars
const CHECKSUM_LEN: usize = 6; // base62(u32::MAX) = 6 chars

pub trait TokenClient: Send + Sync + Clone + 'static {
    // Auth operations
    fn generate_auth_token(&self) -> (String, String);
    fn get_auth_code_expiry_in_seconds(&self) -> u64;

    // Token operations
    fn generate_access_token(&self, token_type: &TokenType) -> (String, String);
    fn validate_token_format(&self, token: &str) -> bool;

    // Code operations
    fn generate_device_code(&self) -> String;
    fn generate_user_code(&self) -> String;
    fn get_device_code_expiry_in_seconds(&self) -> u64;
    fn get_polling_interval_in_seconds(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct TokenClientImpl;

impl TokenClientImpl {
    pub fn new() -> Self {
        Self
    }

    fn generate_url_safe_high_entropic_string(&self) -> String {
        let mut rng = rand::rng();
        let bytes: [u8; 32] = rng.random();
        URL_SAFE_NO_PAD.encode(&bytes)
    }
}

impl TokenClient for TokenClientImpl {
    fn generate_auth_token(&self) -> (String, String) {
        let raw_code = self.generate_url_safe_high_entropic_string();
        let hashed_code = hash_string(&raw_code);
        (raw_code, hashed_code)
    }

    fn get_auth_code_expiry_in_seconds(&self) -> u64 {
        (AUTH_CODE_EXPIRY_MINUTES * 60) as u64
    }

    fn generate_access_token(&self, token_type: &TokenType) -> (String, String) {
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

        let raw_token = format!("{prefix}{body}{checksum}");
        let hashed_token = hash_string(&raw_token);
        (raw_token, hashed_token)
    }

    fn validate_token_format(&self, token: &str) -> bool {
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

    fn generate_device_code(&self) -> String {
        self.generate_url_safe_high_entropic_string()
    }

    fn generate_user_code(&self) -> String {
        let mut rng = rand::rng();
        let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
        (0..6)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect()
    }

    fn get_device_code_expiry_in_seconds(&self) -> u64 {
        (DEVICE_CODE_EXPIRY_MINUTES * 60) as u64
    }

    fn get_polling_interval_in_seconds(&self) -> u64 {
        POLLING_INTERVAL_SECONDS
    }
}

fn base62_encode_padded(value: u128, width: usize) -> String {
    let encoded = base62::encode(value);
    format!("{:0>width$}", encoded, width = width)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> TokenClientImpl {
        TokenClientImpl::new()
    }

    #[test]
    fn test_generated_personal_token_has_correct_format() {
        let c = client();
        let (token, _) = c.generate_access_token(&TokenType::Personal);
        assert!(token.starts_with("gdp_"));
        assert_eq!(token.len(), 54);
        assert!(c.validate_token_format(&token));
    }

    #[test]
    fn test_generated_runner_token_has_correct_format() {
        let c = client();
        let (token, _) = c.generate_access_token(&TokenType::Runner);
        assert!(token.starts_with("gdr_"));
        assert_eq!(token.len(), 54);
        assert!(c.validate_token_format(&token));
    }

    #[test]
    fn test_token_is_alphanumeric() {
        let c = client();
        let (token, _) = c.generate_access_token(&TokenType::Personal);
        let body = &token[4..]; // strip prefix
        assert!(body.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_validate_rejects_bad_prefix() {
        let c = client();
        assert!(!c.validate_token_format("bad_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABB"));
    }

    #[test]
    fn test_validate_rejects_corrupted_checksum() {
        let c = client();
        let (mut token, _) = c.generate_access_token(&TokenType::Personal);
        // Flip last character
        let last = token.pop().unwrap();
        let replacement = if last == 'A' { 'B' } else { 'A' };
        token.push(replacement);
        assert!(!c.validate_token_format(&token));
    }
}
