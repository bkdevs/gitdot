use std::ops::Deref;

use email_address::EmailAddress;

use crate::error::InputError;

/// A validated email address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let email = value.into().trim().to_lowercase();
        if !EmailAddress::is_valid(&email) {
            return Err(InputError::new(field, "must be a valid email address"));
        }
        Ok(Self(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<Email, InputError> {
        Email::parse(value, "email")
    }

    #[test]
    fn accepts_valid_emails() {
        assert_eq!(
            parse("foo@example.com").unwrap().as_ref(),
            "foo@example.com"
        );
        assert_eq!(
            parse("foo+tag@example.com").unwrap().as_ref(),
            "foo+tag@example.com"
        );
    }

    #[test]
    fn sanitizes_whitespace_and_case() {
        assert_eq!(
            parse("  FoO@Example.COM  ").unwrap().as_ref(),
            "foo@example.com"
        );
    }

    #[test]
    fn message_states_the_rule() {
        assert_eq!(
            parse("not-an-email").unwrap_err().to_string(),
            "Invalid email: must be a valid email address"
        );
    }

    #[test]
    fn rejects_invalid() {
        assert!(parse("").is_err());
        assert!(parse("foo@").is_err());
        assert!(parse("@example.com").is_err());
        assert!(parse("foo bar@example.com").is_err());
    }
}
