use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use crate::model::TokenType;

pub fn generate_access_token(token_type: &TokenType) -> String {
    use rand::Rng as _;
    let mut rng = rand::rng();
    let bytes: [u8; 32] = rng.random();

    let prefix = token_type.prefix();
    let body = URL_SAFE_NO_PAD.encode(bytes);
    let crc = crc32fast::hash(&bytes);
    let checksum = URL_SAFE_NO_PAD.encode(crc.to_be_bytes());

    format!("{prefix}{body}{checksum}")
}

const CHECKSUM_LEN: usize = 6; // base64url(4 bytes CRC32) = 6 chars (no pad)

pub fn validate_token_format(token: &str) -> bool {
    let rest = [TokenType::Personal, TokenType::Runner]
        .iter()
        .find_map(|t| token.strip_prefix(t.prefix()));
    let Some(rest) = rest else {
        return false;
    };
    if rest.len() <= CHECKSUM_LEN {
        return false;
    }

    let (body, checksum) = rest.split_at(rest.len() - CHECKSUM_LEN);
    let Ok(body_bytes) = URL_SAFE_NO_PAD.decode(body) else {
        return false;
    };
    let Ok(checksum_bytes) = URL_SAFE_NO_PAD.decode(checksum) else {
        return false;
    };

    let expected_crc = crc32fast::hash(&body_bytes);
    expected_crc.to_be_bytes() == *checksum_bytes
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
        assert_eq!(token.len(), 53);
        assert!(validate_token_format(&token));
    }

    #[test]
    fn test_generated_runner_token_has_correct_format() {
        let token = generate_access_token(&TokenType::Runner);
        assert!(token.starts_with("gdr_"));
        assert_eq!(token.len(), 53);
        assert!(validate_token_format(&token));
    }

    #[test]
    fn test_validate_rejects_bad_prefix() {
        assert!(!validate_token_format(
            "bad_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAbbbbb"
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
