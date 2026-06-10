use std::ops::Deref;

use crate::error::InputError;

/// A validated user code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCode(String);

impl UserCode {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let code = value.into();
        if code.len() != 6 {
            return Err(InputError::new(field, "must be 6 characters"));
        }
        if !code
            .chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='9').contains(&c))
        {
            return Err(InputError::new(
                field,
                "can only contain uppercase letters and the digits 2-9",
            ));
        }
        Ok(Self(code))
    }
}

impl AsRef<str> for UserCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for UserCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(value: &str) -> Result<UserCode, InputError> {
        UserCode::parse(value, "code")
    }

    #[test]
    fn accepts_valid_code() {
        assert_eq!(parse("ABC234").unwrap().as_ref(), "ABC234");
        assert!(parse("ABCDIO").is_ok());
    }

    #[test]
    fn message_names_the_specific_rule() {
        let msg = |v: &str| parse(v).unwrap_err().to_string();
        assert_eq!(msg("ABC23"), "Invalid code: must be 6 characters");
        assert_eq!(msg("ABC2345"), "Invalid code: must be 6 characters");
        assert_eq!(
            msg("ABC230"),
            "Invalid code: can only contain uppercase letters and the digits 2-9"
        );
        assert_eq!(
            msg("ABC-23"),
            "Invalid code: can only contain uppercase letters and the digits 2-9"
        );
    }
}
