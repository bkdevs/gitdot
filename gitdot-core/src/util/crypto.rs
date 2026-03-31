use sha2::{Digest, Sha256};

pub fn hash_string(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    hex::encode(hasher.finalize())
}
