//! Define common structs and constants that can be shared across domains.

mod email;
mod owner;
mod repository;
mod url;

use chrono::{DateTime, Utc};
use nutype::nutype;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use email::Email;
pub use owner::OwnerName;
pub use repository::RepositoryName;
pub use url::Url;

/// TODO: decrease to smaller value
pub const DEFAULT_PER_PAGE_LIMIT: u32 = 10_000;
pub const MAX_PER_PAGE_LIMIT: u32 = 10_000;

/// Keyset pagination cursor.
///
/// On the wire every paginated endpoint accepts/returns this opaque via
/// `util::cursor::{encode, decode}` (base64url-JSON). Internally paginated
/// queries use `ORDER BY created_at DESC, id DESC` and filter rows by
/// `(created_at, id) < (cursor.created_at, cursor.id)`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cursor {
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
}

/// Shared envelope for list-endpoint responses.
///
/// `next_cursor` is `Some` iff more rows exist beyond the current page —
/// the repository fetches `limit + 1` rows internally and emits the cursor
/// derived from the last in-range row when the extra row was returned.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = is_valid_repo_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct RunnerName(String);

#[nutype(
    sanitize(trim),
    validate(predicate = is_valid_filter_name),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct FilterName(String);

#[nutype(
    validate(predicate = is_valid_user_code),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct UserCode(String);

/// Trims each entry of an optional string list and drops the now-empty ones,
/// preserving `None`. Used by request DTOs to clean up user-supplied lists
/// (e.g. commit-filter authors/tags/paths) before persistence.
pub(crate) fn normalize_string_list(values: Option<Vec<String>>) -> Option<Vec<String>> {
    values.map(|v| {
        v.into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    })
}

fn is_valid_repo_slug(s: &str) -> bool {
    validate_slug(s, true).is_ok()
}

fn validate_slug(s: &str, allow_underscore: bool) -> Result<(), &'static str> {
    let is_sep = |c: char| c == '-' || (allow_underscore && c == '_');

    if s.len() < 2 {
        return Err("must be at least 2 characters");
    }
    if s.len() > 32 {
        return Err("must be at most 32 characters");
    }
    if s.chars()
        .any(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || is_sep(c)))
    {
        return Err(if allow_underscore {
            "can only contain lowercase letters, numbers, hyphens, and underscores"
        } else {
            "can only contain lowercase letters, numbers, and hyphens"
        });
    }
    if s.starts_with(is_sep) || s.ends_with(is_sep) {
        return Err(if allow_underscore {
            "cannot start or end with a hyphen or underscore"
        } else {
            "cannot start or end with a hyphen"
        });
    }
    if s.chars()
        .zip(s.chars().skip(1))
        .any(|(a, b)| is_sep(a) && is_sep(b))
    {
        return Err(if allow_underscore {
            "cannot contain consecutive hyphens or underscores"
        } else {
            "cannot contain consecutive hyphens"
        });
    }
    Ok(())
}

fn is_valid_filter_name(s: &str) -> bool {
    !s.is_empty() && s.len() <= 100
}

fn is_valid_user_code(s: &str) -> bool {
    s.len() == 6
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='9').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod user_code {
        use super::*;

        #[test]
        fn valid_code() {
            let code = UserCode::try_new("ABC234").unwrap();
            assert_eq!(code.as_ref(), "ABC234");
        }

        #[test]
        fn rejects_too_short() {
            assert!(UserCode::try_new("ABC23").is_err());
        }

        #[test]
        fn rejects_too_long() {
            assert!(UserCode::try_new("ABC2345").is_err());
        }

        #[test]
        fn rejects_invalid_characters() {
            assert!(UserCode::try_new("ABC230").is_err());
            assert!(UserCode::try_new("ABC231").is_err());
        }

        #[test]
        fn accepts_all_uppercase_letters() {
            assert!(UserCode::try_new("ABCDIO").is_ok());
        }

        #[test]
        fn rejects_special_characters() {
            assert!(UserCode::try_new("ABC-23").is_err());
            assert!(UserCode::try_new("ABC@23").is_err());
        }
    }

    mod normalize_string_list {
        use super::*;

        #[test]
        fn none_stays_none() {
            assert_eq!(normalize_string_list(None), None);
        }

        #[test]
        fn empty_vec_stays_empty() {
            assert_eq!(normalize_string_list(Some(vec![])), Some(vec![]));
        }

        #[test]
        fn trims_each_entry() {
            assert_eq!(
                normalize_string_list(Some(vec!["  alice ".to_string(), "bob".to_string()])),
                Some(vec!["alice".to_string(), "bob".to_string()])
            );
        }

        #[test]
        fn drops_blank_and_whitespace_only_entries() {
            assert_eq!(
                normalize_string_list(Some(vec![
                    "".to_string(),
                    "   ".to_string(),
                    "src/".to_string(),
                ])),
                Some(vec!["src/".to_string()])
            );
        }

        #[test]
        fn all_blank_yields_empty_some() {
            assert_eq!(
                normalize_string_list(Some(vec!["".to_string(), "  ".to_string()])),
                Some(vec![])
            );
        }
    }
}
