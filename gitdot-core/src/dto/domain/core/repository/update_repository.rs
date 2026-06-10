use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

const DESCRIPTION_MAX_LEN: usize = 255;

#[derive(Debug, Clone)]
pub struct UpdateRepositoryRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub description: Option<String>,
    pub readonly: Option<bool>,
}

impl UpdateRepositoryRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        description: Option<String>,
        readonly: Option<bool>,
    ) -> Result<Self, RepositoryError> {
        let description = description
            .map(|d| d.trim().to_string())
            .filter(|d| !d.is_empty());
        if let Some(d) = &description {
            if d.chars().count() > DESCRIPTION_MAX_LEN {
                return Err(InputError::new(
                    "description",
                    format!("must be at most {DESCRIPTION_MAX_LEN} characters"),
                )
                .into());
            }
        }

        if readonly == Some(true) {
            return Err(
                InputError::new("readonly", "a repository cannot be set to readonly").into(),
            );
        }

        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            description,
            readonly,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_setting_readonly_true() {
        let result = UpdateRepositoryRequest::new("johndoe", "my-repo", None, Some(true));
        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn accepts_promote_to_writable() {
        let request =
            UpdateRepositoryRequest::new("johndoe", "my-repo", None, Some(false)).unwrap();
        assert_eq!(request.readonly, Some(false));
    }
}
