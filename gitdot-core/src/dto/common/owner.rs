use std::ops::Deref;

use crate::error::InputError;

use super::validate_slug;

/// A validated user or organization name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerName(String);

impl OwnerName {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let name = value.into().trim().to_lowercase();
        validate_slug(&name, false).map_err(|reason| InputError::new(field, reason))?;
        Ok(Self(name))
    }

    pub(crate) fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for OwnerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for OwnerName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<OwnerName, InputError> {
        OwnerName::parse(value, "owner name")
    }

    #[test]
    fn accepts_valid_names() {
        assert_eq!(parse("johndoe").unwrap().as_ref(), "johndoe");
        assert_eq!(parse("user123").unwrap().as_ref(), "user123");
        assert_eq!(parse("john-doe").unwrap().as_ref(), "john-doe");
    }

    #[test]
    fn sanitizes_whitespace_and_case() {
        assert_eq!(parse("  JohnDoe  ").unwrap().as_ref(), "johndoe");
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
            "Invalid owner name: must be at least 2 characters"
        );
        assert_eq!(
            msg(&"a".repeat(33)),
            "Invalid owner name: must be at most 32 characters"
        );
        assert_eq!(
            msg("john_doe"),
            "Invalid owner name: can only contain lowercase letters, numbers, and hyphens"
        );
        assert_eq!(
            msg("-johndoe"),
            "Invalid owner name: cannot start or end with a hyphen"
        );
        assert_eq!(
            msg("john--doe"),
            "Invalid owner name: cannot contain consecutive hyphens"
        );
    }

    #[test]
    fn rejects_other_invalid_input() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("john@doe").is_err());
        assert!(parse("john doe").is_err());
        assert!(parse("john/doe").is_err());
        assert!(parse("john_doe").is_err());
    }
}
