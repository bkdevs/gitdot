//! Define newtype structs that can be shared across domains.

use nutype::nutype;

fn is_valid_slug(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 100
        && s.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_' || c == '.'
        })
        && !s.starts_with('-')
        && !s.ends_with('-')
}

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = is_valid_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct OwnerName(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(predicate = is_valid_slug),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct RepositoryName(String);

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
        fn valid_with_underscore() {
            let owner = OwnerName::try_new("john_doe").unwrap();
            assert_eq!(owner.as_ref(), "john_doe");
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
            let long_name = "a".repeat(101);
            assert!(OwnerName::try_new(&long_name).is_err());
        }

        #[test]
        fn accepts_max_length() {
            let max_name = "a".repeat(100);
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
            let long_name = "a".repeat(101);
            assert!(RepositoryName::try_new(&long_name).is_err());
        }

        #[test]
        fn accepts_max_length() {
            let max_name = "a".repeat(100);
            assert!(RepositoryName::try_new(&max_name).is_ok());
        }
    }
}
