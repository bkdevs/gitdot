use std::ops::Deref;

use crate::error::InputError;

use super::validate_slug;

/// A validated CI runner name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerName(String);

impl RunnerName {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let name = value.into().trim().to_lowercase();
        validate_slug(&name, true).map_err(|reason| InputError::new(field, reason))?;
        Ok(Self(name))
    }
}

impl AsRef<str> for RunnerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for RunnerName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<RunnerName, InputError> {
        RunnerName::parse(value, "runner name")
    }

    #[test]
    fn accepts_valid_names() {
        assert_eq!(parse("my-runner").unwrap().as_ref(), "my-runner");
        assert_eq!(parse("runner_1").unwrap().as_ref(), "runner_1");
    }

    #[test]
    fn sanitizes_whitespace_and_case() {
        assert_eq!(parse("  My-Runner  ").unwrap().as_ref(), "my-runner");
    }

    #[test]
    fn message_names_the_specific_rule() {
        let msg = |v: &str| parse(v).unwrap_err().to_string();
        assert_eq!(
            msg("a"),
            "Invalid runner name: must be at least 2 characters"
        );
        assert_eq!(
            msg("my runner"),
            "Invalid runner name: can only contain lowercase letters, numbers, hyphens, and underscores"
        );
        assert_eq!(
            msg("_runner"),
            "Invalid runner name: cannot start or end with a hyphen or underscore"
        );
    }

    #[test]
    fn rejects_other_invalid_input() {
        assert!(parse("").is_err());
        assert!(parse("my@runner").is_err());
        assert!(parse("a__b").is_err());
    }
}
