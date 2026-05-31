use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use hmac::{Hmac, Mac};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rand::RngExt as _;
use serde::Serialize;
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    dto::{GitdotClaims, OAuthStatePayload, UserMetadata},
    error::TokenError,
    model::TokenType,
    util::{auth::GITDOT_SERVER_ID, crypto::hash_string},
};

const AUTH_CODE_EXPIRY_MINUTES: i64 = 10;
const ACCESS_TOKEN_EXPIRY_SECONDS: u64 = 3600;
const REFRESH_TOKEN_EXPIRY_SECONDS: u64 = 30 * 24 * 3600;
const OAUTH_STATE_EXPIRY_SECONDS: u64 = 600;
const DEVICE_CODE_EXPIRY_MINUTES: i64 = 10;
const POLLING_INTERVAL_SECONDS: u64 = 1;

const BODY_HALF_LEN: usize = 22; // base62(u128::MAX) = 22 chars
const BODY_LEN: usize = BODY_HALF_LEN * 2; // two u128 halves = 44 chars
const CHECKSUM_LEN: usize = 6; // base62(u32::MAX) = 6 chars

/// Generates and validates the various opaque codes, access tokens, OAuth
/// state, and signed JWTs used across gitdot's auth flows, and exposes the
/// canonical expiry/polling durations those flows depend on.
///
/// Secret material (auth codes, access tokens) is returned as a
/// `(raw, hashed)` pair: the raw value is shown to the caller once and the
/// SHA-hash is what gets persisted, so the plaintext is never stored.
pub trait TokenClient: Send + Sync + Clone + 'static {
    // Code operations

    /// Generates a high-entropy, single-use code (e.g. an authorization code),
    /// returning `(raw_code, hashed_code)`. Persist the hash; return the raw to
    /// the caller once.
    fn generate_high_entropic_code(&self) -> (String, String);

    /// Generates a short, human-readable code (6 uppercase chars from an
    /// unambiguous alphabet) for device-flow user codes and email login codes,
    /// returning `(raw_code, hashed_code)`. Show the raw code to the user;
    /// persist only the hash.
    fn generate_readable_code(&self) -> (String, String);

    // Expiry operations

    /// Lifetime of an authorization code, in seconds.
    fn get_auth_code_expiry_in_seconds(&self) -> u64;

    /// Lifetime of an access token, in seconds.
    fn get_access_token_expiry_in_seconds(&self) -> u64;

    /// Lifetime of a refresh token, in seconds.
    fn get_refresh_token_expiry_in_seconds(&self) -> u64;

    /// Lifetime of a device code, in seconds.
    fn get_device_code_expiry_in_seconds(&self) -> u64;

    /// Interval, in seconds, that device-flow clients should wait between polls.
    fn get_polling_interval_in_seconds(&self) -> u64;

    // Token operations

    /// Generates an access token of the given `token_type`, returning
    /// `(raw_token, hashed_token)`. The raw token carries the type prefix and a
    /// CRC32 checksum (verifiable via [`validate_token_format`]); persist the
    /// hash.
    ///
    /// [`validate_token_format`]: TokenClient::validate_token_format
    fn generate_access_token(&self, token_type: &TokenType) -> (String, String);

    /// Checks that `token` has a recognized type prefix, the expected length,
    /// and a valid embedded CRC32 checksum. This is a cheap, offline format
    /// check — it does not prove the token was ever issued.
    fn validate_token_format(&self, token: &str) -> bool;

    // OAuth state operations

    /// Generates an HMAC-signed OAuth state value embedding a nonce and an
    /// expiry, for CSRF protection across the OAuth redirect.
    fn generate_oauth_state(&self) -> String;

    /// Verifies an OAuth `state` value's signature and expiry.
    ///
    /// # Errors
    /// Returns `Err(message)` if the state is malformed, the signature does not
    /// verify, or it has expired.
    fn verify_oauth_state(&self, state: &str) -> Result<(), String>;

    // JWT operations

    /// Signs arbitrary `claims` into an EdDSA JWT using gitdot's private key.
    ///
    /// # Errors
    /// - [`TokenError::SigningError`] — the key is invalid or signing failed.
    fn generate_jwt<T: Serialize + Send + Sync>(&self, claims: &T) -> Result<String, TokenError>;

    /// Builds and signs a gitdot session JWT for `user_id`/`username` with the
    /// standard issuer, audience, and access-token expiry.
    ///
    /// # Errors
    /// - [`TokenError::SigningError`] — the key is invalid or signing failed.
    fn generate_gitdot_jwt(&self, user_id: Uuid, username: &str) -> Result<String, TokenError>;
}

#[derive(Debug, Clone)]
pub struct TokenClientImpl {
    gitdot_private_key: String,
}

impl TokenClientImpl {
    pub fn new(gitdot_private_key: String) -> Self {
        Self { gitdot_private_key }
    }

    fn base62_encode_padded(&self, value: u128, width: usize) -> String {
        let encoded = base62::encode(value);
        format!("{:0>width$}", encoded, width = width)
    }
}

impl TokenClient for TokenClientImpl {
    fn generate_high_entropic_code(&self) -> (String, String) {
        let mut rng = rand::rng();
        let bytes: [u8; 32] = rng.random();
        let raw_code = URL_SAFE_NO_PAD.encode(&bytes);
        let hashed_code = hash_string(&raw_code);
        (raw_code, hashed_code)
    }

    fn generate_readable_code(&self) -> (String, String) {
        let mut rng = rand::rng();
        let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
        let raw_code: String = (0..6)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect();
        let hashed_code = hash_string(&raw_code);
        (raw_code, hashed_code)
    }

    fn get_auth_code_expiry_in_seconds(&self) -> u64 {
        (AUTH_CODE_EXPIRY_MINUTES * 60) as u64
    }

    fn get_access_token_expiry_in_seconds(&self) -> u64 {
        ACCESS_TOKEN_EXPIRY_SECONDS
    }

    fn get_refresh_token_expiry_in_seconds(&self) -> u64 {
        REFRESH_TOKEN_EXPIRY_SECONDS
    }

    fn get_device_code_expiry_in_seconds(&self) -> u64 {
        (DEVICE_CODE_EXPIRY_MINUTES * 60) as u64
    }

    fn get_polling_interval_in_seconds(&self) -> u64 {
        POLLING_INTERVAL_SECONDS
    }

    fn generate_access_token(&self, token_type: &TokenType) -> (String, String) {
        let mut rng = rand::rng();
        let bytes: [u8; 32] = rng.random();

        let prefix = token_type.prefix();
        let hi = u128::from_be_bytes(bytes[..16].try_into().unwrap());
        let lo = u128::from_be_bytes(bytes[16..].try_into().unwrap());
        let body = format!(
            "{}{}",
            self.base62_encode_padded(hi, BODY_HALF_LEN),
            self.base62_encode_padded(lo, BODY_HALF_LEN)
        );
        let crc = crc32fast::hash(&bytes);
        let checksum = self.base62_encode_padded(crc as u128, CHECKSUM_LEN);

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

    fn generate_oauth_state(&self) -> String {
        let payload = OAuthStatePayload {
            nonce: Uuid::new_v4().to_string(),
            exp: Utc::now().timestamp() as u64 + OAUTH_STATE_EXPIRY_SECONDS,
        };
        let payload_json = serde_json::to_string(&payload).unwrap();
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

        let mut mac = Hmac::<Sha256>::new_from_slice(self.gitdot_private_key.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(payload_b64.as_bytes());
        let sig = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

        format!("{payload_b64}.{sig}")
    }

    fn verify_oauth_state(&self, state: &str) -> Result<(), String> {
        let (payload_b64, sig_b64) = state.split_once('.').ok_or("Invalid state format")?;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.gitdot_private_key.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(payload_b64.as_bytes());

        let sig = URL_SAFE_NO_PAD
            .decode(sig_b64)
            .map_err(|_| "Invalid signature encoding")?;
        mac.verify_slice(&sig).map_err(|_| "Invalid signature")?;

        let payload_json = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|_| "Invalid payload encoding")?;
        let payload: OAuthStatePayload =
            serde_json::from_slice(&payload_json).map_err(|_| "Invalid payload")?;

        if payload.exp < Utc::now().timestamp() as u64 {
            return Err("State expired".to_string());
        }

        Ok(())
    }

    fn generate_jwt<T: Serialize + Send + Sync>(&self, claims: &T) -> Result<String, TokenError> {
        let encoding_key = EncodingKey::from_ed_pem(self.gitdot_private_key.as_bytes())
            .map_err(|e| TokenError::SigningError(e.to_string()))?;
        encode(&Header::new(Algorithm::EdDSA), claims, &encoding_key)
            .map_err(|e| TokenError::SigningError(e.to_string()))
    }

    fn generate_gitdot_jwt(&self, user_id: Uuid, username: &str) -> Result<String, TokenError> {
        let now = Utc::now().timestamp() as usize;
        let claims = GitdotClaims {
            iss: GITDOT_SERVER_ID.to_string(),
            aud: vec![GITDOT_SERVER_ID.to_string()],
            sub: user_id.to_string(),
            iat: now,
            exp: now + ACCESS_TOKEN_EXPIRY_SECONDS as usize,
            user_metadata: UserMetadata {
                username: username.to_string(),
            },
        };
        self.generate_jwt(&claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client() -> TokenClientImpl {
        TokenClientImpl::new(String::new())
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

    #[test]
    fn test_readable_code_uses_six_unambiguous_chars() {
        let c = client();
        let (raw, _) = c.generate_readable_code();
        assert_eq!(raw.chars().count(), 6);

        const READABLE_ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        assert!(
            raw.chars().all(|ch| READABLE_ALPHABET.contains(ch)),
            "raw code {raw:?} contains a char outside the unambiguous alphabet",
        );
        // The unambiguous alphabet excludes 0/O/1/I to avoid transcription errors.
        assert!(!raw.contains(['0', 'O', '1', 'I']));
    }

    #[test]
    fn test_readable_code_hash_matches_raw() {
        let c = client();
        let (raw, hashed) = c.generate_readable_code();
        // The persisted hash is the SHA-256 hex of the raw code, never the code itself.
        assert_eq!(hashed, hash_string(&raw));
        assert_ne!(hashed, raw);
        assert_eq!(hashed.len(), 64);
        assert!(hashed.chars().all(|ch| ch.is_ascii_hexdigit()));
    }

    #[test]
    fn test_readable_codes_are_random() {
        let c = client();
        // 32^6 space, so 100 draws colliding would be astronomically unlikely.
        let codes: std::collections::HashSet<String> =
            (0..100).map(|_| c.generate_readable_code().0).collect();
        assert!(
            codes.len() > 90,
            "expected mostly-distinct codes, got {}",
            codes.len()
        );
    }
}
