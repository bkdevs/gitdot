use std::ops::Deref;

use crate::error::InputError;

use super::validate_slug;

/// A validated repository name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryName(String);

impl RepositoryName {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let name = strip_git_suffix(value.into().trim().to_lowercase());
        validate_slug(&name, true).map_err(|reason| InputError::new(field, reason))?;
        Ok(Self(name))
    }

    pub(crate) fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for RepositoryName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for RepositoryName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn strip_git_suffix(s: String) -> String {
    s.strip_suffix(".git").map(|s| s.to_string()).unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<RepositoryName, InputError> {
        RepositoryName::parse(value, "repository name")
    }

    #[test]
    fn accepts_valid_names() {
        assert_eq!(parse("myrepo").unwrap().as_ref(), "myrepo");
        assert_eq!(parse("repo123").unwrap().as_ref(), "repo123");
        assert_eq!(parse("my-repo").unwrap().as_ref(), "my-repo");
        assert_eq!(parse("my_repo").unwrap().as_ref(), "my_repo");
    }

    #[test]
    fn sanitizes_whitespace_case_and_git_suffix() {
        assert_eq!(parse("  MyRepo  ").unwrap().as_ref(), "myrepo");
        assert_eq!(parse("myrepo.git").unwrap().as_ref(), "myrepo");
    }

    #[test]
    fn accepts_boundary_lengths() {
        assert!(parse(&"a".repeat(2)).is_ok());
        assert!(parse(&"a".repeat(32)).is_ok());
    }

    #[test]
    fn message_names_the_specific_rule() {
        let msg = |v: &str| parse(v).unwrap_err().to_string();
        assert_eq!(
            msg("a"),
            "Invalid repository name: must be at least 2 characters"
        );
        assert_eq!(
            msg(&"a".repeat(33)),
            "Invalid repository name: must be at most 32 characters"
        );
        assert_eq!(
            msg("my repo"),
            "Invalid repository name: can only contain lowercase letters, numbers, hyphens, and underscores"
        );
        assert_eq!(
            msg("_myrepo"),
            "Invalid repository name: cannot start or end with a hyphen or underscore"
        );
        assert_eq!(
            msg("my__repo"),
            "Invalid repository name: cannot contain consecutive hyphens or underscores"
        );
    }

    #[test]
    fn rejects_other_invalid_input() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("my@repo").is_err());
        assert!(parse("my/repo").is_err());
        assert!(parse("my--repo").is_err());
    }
}
