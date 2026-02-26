use nutype::nutype;
use uuid::Uuid;

use crate::error::TokenError;

#[derive(Debug, Clone)]
pub struct AuthorizeDeviceRequest {
    pub user_code: UserCode,
    pub user_id: Uuid,
}

impl AuthorizeDeviceRequest {
    pub fn new(user_code: &str, user_id: Uuid) -> Result<Self, TokenError> {
        Ok(Self {
            user_code: UserCode::try_new(user_code)
                .map_err(|e| TokenError::InvalidUserCode(e.to_string()))?,
            user_id,
        })
    }
}

fn is_valid_user_code(s: &str) -> bool {
    s.len() == 6
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='9').contains(&c))
}

#[nutype(
    validate(predicate = is_valid_user_code),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct UserCode(String);

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

    mod authorize_device_request {
        use super::*;

        #[test]
        fn valid_request() {
            let user_id = Uuid::new_v4();
            let request = AuthorizeDeviceRequest::new("ABC234", user_id).unwrap();

            assert_eq!(request.user_code.as_ref(), "ABC234");
            assert_eq!(request.user_id, user_id);
        }

        #[test]
        fn rejects_invalid_code() {
            let user_id = Uuid::new_v4();
            let result = AuthorizeDeviceRequest::new("invalid", user_id);

            assert!(matches!(result, Err(TokenError::InvalidUserCode(_))));
        }
    }
}
