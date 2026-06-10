use std::ops::Deref;

use crate::error::InputError;

/// A validated URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url(String);

impl Url {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let url = value.into().trim().to_string();
        if ::url::Url::parse(&url).is_err() {
            return Err(InputError::new(field, "must be a valid URL"));
        }
        Ok(Self(url))
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Url {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<Url, InputError> {
        Url::parse(value, "url")
    }

    #[test]
    fn accepts_valid_urls() {
        assert_eq!(
            parse("https://example.com/webhook").unwrap().as_ref(),
            "https://example.com/webhook"
        );
        assert_eq!(
            parse("http://localhost:8080/hook").unwrap().as_ref(),
            "http://localhost:8080/hook"
        );
    }

    #[test]
    fn sanitizes_whitespace() {
        assert_eq!(
            parse("  https://example.com  ").unwrap().as_ref(),
            "https://example.com"
        );
    }

    #[test]
    fn message_states_the_rule() {
        assert_eq!(
            parse("not-a-url").unwrap_err().to_string(),
            "Invalid url: must be a valid URL"
        );
    }

    #[test]
    fn rejects_invalid() {
        assert!(parse("").is_err());
        assert!(parse("not-a-url").is_err());
        assert!(parse("example.com/webhook").is_err());
    }
}
