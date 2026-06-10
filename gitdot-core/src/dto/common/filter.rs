use std::ops::Deref;

use crate::error::InputError;

/// A validated commit-filter name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterName(String);

impl FilterName {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let name = value.into().trim().to_string();
        if name.is_empty() {
            return Err(InputError::new(field, "cannot be empty"));
        }
        if name.len() > 100 {
            return Err(InputError::new(field, "must be at most 100 characters"));
        }
        Ok(Self(name))
    }
}

impl AsRef<str> for FilterName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for FilterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<FilterName, InputError> {
        FilterName::parse(value, "filter name")
    }

    #[test]
    fn accepts_valid_names() {
        assert_eq!(parse("My Filter").unwrap().as_ref(), "My Filter");
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(parse("  My Filter  ").unwrap().as_ref(), "My Filter");
    }

    #[test]
    fn accepts_max_length() {
        assert!(parse(&"a".repeat(100)).is_ok());
    }

    #[test]
    fn message_names_the_specific_rule() {
        let msg = |v: &str| parse(v).unwrap_err().to_string();
        assert_eq!(msg(""), "Invalid filter name: cannot be empty");
        assert_eq!(msg("   "), "Invalid filter name: cannot be empty");
        assert_eq!(
            msg(&"a".repeat(101)),
            "Invalid filter name: must be at most 100 characters"
        );
    }
}
