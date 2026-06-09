//! Define common structs and constants that can be shared across domains.

use chrono::{DateTime, Utc};
use email_address::EmailAddress;
use nutype::nutype;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    validate(predicate = is_valid_owner_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct OwnerName(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = is_valid_repo_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct RunnerName(String);

#[nutype(
    sanitize(trim, lowercase, with = strip_git_suffix),
    validate(predicate = is_valid_repo_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct RepositoryName(String);

#[nutype(
    sanitize(trim),
    validate(predicate = is_valid_url),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct WebhookUrl(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = is_valid_email),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub(crate) struct Email(String);

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

fn is_valid_owner_slug(s: &str) -> bool {
    is_valid_slug(s, false)
}

fn is_valid_repo_slug(s: &str) -> bool {
    is_valid_slug(s, true)
}

fn is_valid_slug(s: &str, allow_underscore: bool) -> bool {
    let is_sep = |c: char| c == '-' || (allow_underscore && c == '_');
    let is_allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || is_sep(c);

    (2..=32).contains(&s.len())
        && s.chars().all(is_allowed)
        && !s.starts_with(is_sep)
        && !s.ends_with(is_sep)
        && !s
            .chars()
            .zip(s.chars().skip(1))
            .any(|(a, b)| is_sep(a) && is_sep(b))
}

fn strip_git_suffix(s: String) -> String {
    s.strip_suffix(".git").map(|s| s.to_string()).unwrap_or(s)
}

fn is_valid_url(s: &str) -> bool {
    url::Url::parse(s).is_ok()
}

fn is_valid_email(s: &str) -> bool {
    EmailAddress::is_valid(s)
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

    mod owner_name {
        use super::*;

        #[test]
        fn valid_lowercase() {
            let owner = OwnerName::try_new("johndoe").unwrap();
            assert_eq!(owner.as_ref(), "johndoe");
        }

        #[test]
        fn valid_with_numbers() {
            let owner = OwnerName::try_new("user123").unwrap();
            assert_eq!(owner.as_ref(), "user123");
        }

        #[test]
        fn valid_with_hyphen() {
            let owner = OwnerName::try_new("john-doe").unwrap();
            assert_eq!(owner.as_ref(), "john-doe");
        }

        #[test]
        fn rejects_underscore() {
            assert!(OwnerName::try_new("john_doe").is_err());
            assert!(OwnerName::try_new("_johndoe").is_err());
            assert!(OwnerName::try_new("johndoe_").is_err());
        }

        #[test]
        fn rejects_consecutive_separators() {
            assert!(OwnerName::try_new("john--doe").is_err());
        }

        #[test]
        fn sanitizes_uppercase_to_lowercase() {
            let owner = OwnerName::try_new("JohnDoe").unwrap();
            assert_eq!(owner.as_ref(), "johndoe");
        }

        #[test]
        fn sanitizes_whitespace() {
            let owner = OwnerName::try_new("  johndoe  ").unwrap();
            assert_eq!(owner.as_ref(), "johndoe");
        }

        #[test]
        fn rejects_empty_string() {
            assert!(OwnerName::try_new("").is_err());
        }

        #[test]
        fn rejects_whitespace_only() {
            assert!(OwnerName::try_new("   ").is_err());
        }

        #[test]
        fn rejects_special_characters() {
            assert!(OwnerName::try_new("john@doe").is_err());
            assert!(OwnerName::try_new("john.doe").is_err());
            assert!(OwnerName::try_new("john/doe").is_err());
            assert!(OwnerName::try_new("john doe").is_err());
        }

        #[test]
        fn rejects_starting_with_hyphen() {
            assert!(OwnerName::try_new("-johndoe").is_err());
        }

        #[test]
        fn rejects_ending_with_hyphen() {
            assert!(OwnerName::try_new("johndoe-").is_err());
        }

        #[test]
        fn rejects_too_long() {
            let long_name = "a".repeat(33);
            assert!(OwnerName::try_new(&long_name).is_err());
        }

        #[test]
        fn accepts_max_length() {
            let max_name = "a".repeat(32);
            assert!(OwnerName::try_new(&max_name).is_ok());
        }
    }

    mod repository_name {
        use super::*;

        #[test]
        fn valid_lowercase() {
            let repo = RepositoryName::try_new("myrepo").unwrap();
            assert_eq!(repo.as_ref(), "myrepo");
        }

        #[test]
        fn valid_with_numbers() {
            let repo = RepositoryName::try_new("repo123").unwrap();
            assert_eq!(repo.as_ref(), "repo123");
        }

        #[test]
        fn valid_with_hyphen() {
            let repo = RepositoryName::try_new("my-repo").unwrap();
            assert_eq!(repo.as_ref(), "my-repo");
        }

        #[test]
        fn valid_with_underscore() {
            let repo = RepositoryName::try_new("my_repo").unwrap();
            assert_eq!(repo.as_ref(), "my_repo");
        }

        #[test]
        fn rejects_edge_underscore() {
            assert!(RepositoryName::try_new("_myrepo").is_err());
            assert!(RepositoryName::try_new("myrepo_").is_err());
            assert!(RepositoryName::try_new("__").is_err());
        }

        #[test]
        fn rejects_consecutive_separators() {
            assert!(RepositoryName::try_new("my__repo").is_err());
            assert!(RepositoryName::try_new("my--repo").is_err());
            assert!(RepositoryName::try_new("my-_repo").is_err());
        }

        #[test]
        fn sanitizes_uppercase_to_lowercase() {
            let repo = RepositoryName::try_new("MyRepo").unwrap();
            assert_eq!(repo.as_ref(), "myrepo");
        }

        #[test]
        fn sanitizes_whitespace() {
            let repo = RepositoryName::try_new("  myrepo  ").unwrap();
            assert_eq!(repo.as_ref(), "myrepo");
        }

        #[test]
        fn rejects_empty_string() {
            assert!(RepositoryName::try_new("").is_err());
        }

        #[test]
        fn rejects_whitespace_only() {
            assert!(RepositoryName::try_new("   ").is_err());
        }

        #[test]
        fn rejects_special_characters() {
            assert!(RepositoryName::try_new("my@repo").is_err());
            assert!(RepositoryName::try_new("my.repo").is_err());
            assert!(RepositoryName::try_new("my/repo").is_err());
            assert!(RepositoryName::try_new("my repo").is_err());
        }

        #[test]
        fn rejects_starting_with_hyphen() {
            assert!(RepositoryName::try_new("-myrepo").is_err());
        }

        #[test]
        fn rejects_ending_with_hyphen() {
            assert!(RepositoryName::try_new("myrepo-").is_err());
        }

        #[test]
        fn rejects_too_long() {
            let long_name = "a".repeat(33);
            assert!(RepositoryName::try_new(&long_name).is_err());
        }

        #[test]
        fn accepts_max_length() {
            let max_name = "a".repeat(32);
            assert!(RepositoryName::try_new(&max_name).is_ok());
        }

        #[test]
        fn strips_git_suffix() {
            let repo = RepositoryName::try_new("myrepo.git").unwrap();
            assert_eq!(repo.as_ref(), "myrepo");
        }
    }

    mod email {
        use super::*;

        #[test]
        fn valid_simple() {
            let e = Email::try_new("foo@example.com").unwrap();
            assert_eq!(e.as_ref(), "foo@example.com");
        }

        #[test]
        fn valid_with_plus() {
            let e = Email::try_new("foo+tag@example.com").unwrap();
            assert_eq!(e.as_ref(), "foo+tag@example.com");
        }

        #[test]
        fn valid_with_dot_in_local() {
            let e = Email::try_new("first.last@example.com").unwrap();
            assert_eq!(e.as_ref(), "first.last@example.com");
        }

        #[test]
        fn valid_with_subdomain() {
            let e = Email::try_new("u@mail.eng.example.com").unwrap();
            assert_eq!(e.as_ref(), "u@mail.eng.example.com");
        }

        #[test]
        fn valid_with_hyphen_in_domain() {
            let e = Email::try_new("u@my-host.example.com").unwrap();
            assert_eq!(e.as_ref(), "u@my-host.example.com");
        }

        #[test]
        fn valid_with_special_chars_in_local() {
            assert!(Email::try_new("a!#$%&'*+-/=?^_`{|}~b@example.com").is_ok());
        }

        #[test]
        fn sanitizes_uppercase_to_lowercase() {
            let e = Email::try_new("FoO@Example.COM").unwrap();
            assert_eq!(e.as_ref(), "foo@example.com");
        }

        #[test]
        fn sanitizes_surrounding_whitespace() {
            let e = Email::try_new("  foo@example.com  ").unwrap();
            assert_eq!(e.as_ref(), "foo@example.com");
        }

        #[test]
        fn rejects_empty() {
            assert!(Email::try_new("").is_err());
        }

        #[test]
        fn rejects_whitespace_only() {
            assert!(Email::try_new("   ").is_err());
        }

        #[test]
        fn rejects_missing_at() {
            assert!(Email::try_new("fooexample.com").is_err());
        }

        #[test]
        fn rejects_multiple_at() {
            assert!(Email::try_new("foo@bar@example.com").is_err());
        }

        #[test]
        fn rejects_missing_local() {
            assert!(Email::try_new("@example.com").is_err());
        }

        #[test]
        fn rejects_missing_domain() {
            assert!(Email::try_new("foo@").is_err());
        }

        #[test]
        fn accepts_domain_without_dot() {
            // RFC-permissive: bare hostnames like `localhost` are legal mail
            // destinations, so the crate accepts them.
            assert!(Email::try_new("foo@localhost").is_ok());
        }

        #[test]
        fn rejects_leading_dot_in_local() {
            assert!(Email::try_new(".foo@example.com").is_err());
        }

        #[test]
        fn rejects_trailing_dot_in_local() {
            assert!(Email::try_new("foo.@example.com").is_err());
        }

        #[test]
        fn rejects_consecutive_dots_in_local() {
            assert!(Email::try_new("foo..bar@example.com").is_err());
        }

        #[test]
        fn rejects_leading_dot_in_domain() {
            assert!(Email::try_new("foo@.example.com").is_err());
        }

        #[test]
        fn rejects_trailing_dot_in_domain() {
            assert!(Email::try_new("foo@example.com.").is_err());
        }

        #[test]
        fn rejects_label_starting_with_hyphen() {
            assert!(Email::try_new("foo@-example.com").is_err());
        }

        #[test]
        fn rejects_label_ending_with_hyphen() {
            assert!(Email::try_new("foo@example-.com").is_err());
        }

        #[test]
        fn accepts_underscore_in_domain() {
            // RFC 5322 permits underscores in the domain part even though
            // RFC 1035 hostnames do not.
            assert!(Email::try_new("foo@bad_host.com").is_ok());
        }

        #[test]
        fn rejects_space_in_address() {
            assert!(Email::try_new("foo bar@example.com").is_err());
            assert!(Email::try_new("foo@example .com").is_err());
        }

        #[test]
        fn accepts_tld_one_char() {
            // The RFC doesn't actually forbid single-char TLDs.
            assert!(Email::try_new("foo@example.x").is_ok());
        }

        #[test]
        fn accepts_numeric_tld() {
            // Numeric TLDs are unusual but RFC-legal.
            assert!(Email::try_new("foo@example.123").is_ok());
        }

        #[test]
        fn rejects_local_too_long() {
            let local = "a".repeat(65);
            assert!(Email::try_new(format!("{local}@example.com").as_str()).is_err());
        }

        #[test]
        fn accepts_local_max_length() {
            let local = "a".repeat(64);
            assert!(Email::try_new(format!("{local}@example.com").as_str()).is_ok());
        }

        #[test]
        fn rejects_label_too_long() {
            let label = "a".repeat(64);
            assert!(Email::try_new(format!("foo@{label}.com").as_str()).is_err());
        }
    }

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

    mod webhook_url {
        use super::*;

        #[test]
        fn valid_https() {
            let url = WebhookUrl::try_new("https://example.com/webhook").unwrap();
            assert_eq!(url.as_ref(), "https://example.com/webhook");
        }

        #[test]
        fn valid_http() {
            let url = WebhookUrl::try_new("http://localhost:8080/hook").unwrap();
            assert_eq!(url.as_ref(), "http://localhost:8080/hook");
        }

        #[test]
        fn sanitizes_whitespace() {
            let url = WebhookUrl::try_new("  https://example.com  ").unwrap();
            assert_eq!(url.as_ref(), "https://example.com");
        }

        #[test]
        fn rejects_empty_string() {
            assert!(WebhookUrl::try_new("").is_err());
        }

        #[test]
        fn rejects_not_a_url() {
            assert!(WebhookUrl::try_new("not-a-url").is_err());
        }

        #[test]
        fn rejects_missing_scheme() {
            assert!(WebhookUrl::try_new("example.com/webhook").is_err());
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
