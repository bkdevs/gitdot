use std::ops::Deref;

use crate::error::InputError;

/// A validated git smart-HTTP service name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitService(String);

impl GitService {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let service = value.into();
        if service != "git-upload-pack" && service != "git-receive-pack" {
            return Err(InputError::new(
                field,
                "must be git-upload-pack or git-receive-pack",
            ));
        }
        Ok(Self(service))
    }
}

impl AsRef<str> for GitService {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for GitService {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A validated git smart-HTTP request content type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitContentType(String);

impl GitContentType {
    pub(crate) fn parse(value: impl Into<String>, field: &'static str) -> Result<Self, InputError> {
        let content_type = value.into();
        if content_type != "application/x-git-upload-pack-request"
            && content_type != "application/x-git-receive-pack-request"
        {
            return Err(InputError::new(
                field,
                "must be a git upload-pack or receive-pack request content type",
            ));
        }
        Ok(Self(content_type))
    }
}

impl AsRef<str> for GitContentType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for GitContentType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod git_service {
        use super::*;

        #[test]
        fn accepts_valid_services() {
            assert_eq!(
                GitService::parse("git-upload-pack", "service")
                    .unwrap()
                    .as_ref(),
                "git-upload-pack"
            );
            assert_eq!(
                GitService::parse("git-receive-pack", "service")
                    .unwrap()
                    .as_ref(),
                "git-receive-pack"
            );
        }

        #[test]
        fn rejects_invalid_services() {
            assert!(GitService::parse("git-fetch", "service").is_err());
            assert!(GitService::parse("upload-pack", "service").is_err());
            assert!(GitService::parse("", "service").is_err());
            assert!(GitService::parse("git-upload-pack ", "service").is_err());
        }

        #[test]
        fn message_states_the_rule() {
            assert_eq!(
                GitService::parse("git-fetch", "service")
                    .unwrap_err()
                    .to_string(),
                "Invalid service: must be git-upload-pack or git-receive-pack"
            );
        }
    }

    mod git_content_type {
        use super::*;

        #[test]
        fn accepts_valid_content_types() {
            assert!(
                GitContentType::parse("application/x-git-upload-pack-request", "content type")
                    .is_ok()
            );
            assert!(
                GitContentType::parse("application/x-git-receive-pack-request", "content type")
                    .is_ok()
            );
        }

        #[test]
        fn rejects_invalid_content_types() {
            assert!(GitContentType::parse("text/plain", "content type").is_err());
            assert!(GitContentType::parse("application/json", "content type").is_err());
            assert!(GitContentType::parse("", "content type").is_err());
        }
    }
}
