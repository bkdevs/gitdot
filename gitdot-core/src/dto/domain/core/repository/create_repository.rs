use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
    model::{RepositoryOwnerType, RepositoryVisibility},
};

#[derive(Debug, Clone)]
pub struct CreateRepositoryRequest {
    pub name: RepositoryName,
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub visibility: RepositoryVisibility,
    pub description: Option<String>,
    pub init_readme: bool,
    pub gitignore: Option<GitignoreTemplate>,
    pub license: Option<LicenseTemplate>,
}

impl CreateRepositoryRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        repo_name: &str,
        user_id: Uuid,
        owner_name: &str,
        owner_type: &str,
        visibility: &str,
        description: Option<&str>,
        init_readme: bool,
        gitignore: Option<&str>,
        license: Option<&str>,
    ) -> Result<Self, RepositoryError> {
        let description = description
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);
        let gitignore = gitignore.map(GitignoreTemplate::try_from).transpose()?;
        let license = license.map(LicenseTemplate::try_from).transpose()?;
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            user_id,
            owner_name: OwnerName::parse(owner_name, "owner name")?,
            owner_type: owner_type.try_into()?,
            visibility: visibility.try_into()?,
            description,
            init_readme,
            gitignore,
            license,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitignoreTemplate {
    Rust,
    Node,
    Python,
    Go,
}

impl TryFrom<&str> for GitignoreTemplate {
    type Error = RepositoryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "rust" => Ok(GitignoreTemplate::Rust),
            "node" => Ok(GitignoreTemplate::Node),
            "python" => Ok(GitignoreTemplate::Python),
            "go" => Ok(GitignoreTemplate::Go),
            _ => Err(InputError::new("gitignore template", value).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseTemplate {
    Mit,
    Apache2,
}

impl TryFrom<&str> for LicenseTemplate {
    type Error = RepositoryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "mit" => Ok(LicenseTemplate::Mit),
            "apache-2.0" => Ok(LicenseTemplate::Apache2),
            _ => Err(InputError::new("license template", value).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(user_id: Uuid) -> Result<CreateRepositoryRequest, RepositoryError> {
        CreateRepositoryRequest::new(
            "my-repo", user_id, "johndoe", "user", "public", None, false, None, None,
        )
    }

    #[test]
    fn valid_request() {
        let user_id = Uuid::new_v4();
        let request = req(user_id).unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.owner_name.as_ref(), "johndoe");
        assert_eq!(request.owner_type, RepositoryOwnerType::User);
        assert_eq!(request.visibility, RepositoryVisibility::Public);
        assert_eq!(request.description, None);
        assert!(!request.init_readme);
        assert_eq!(request.gitignore, None);
        assert_eq!(request.license, None);
    }

    #[test]
    fn valid_private_org_repository() {
        let user_id = Uuid::new_v4();
        let request = CreateRepositoryRequest::new(
            "repo",
            user_id,
            "myorg",
            "organization",
            "private",
            None,
            false,
            None,
            None,
        )
        .unwrap();

        assert_eq!(request.owner_type, RepositoryOwnerType::Organization);
        assert_eq!(request.visibility, RepositoryVisibility::Private);
    }

    #[test]
    fn strips_git_suffix_from_repo_name() {
        let user_id = Uuid::new_v4();
        let request = CreateRepositoryRequest::new(
            "my-repo.git",
            user_id,
            "johndoe",
            "user",
            "public",
            None,
            false,
            None,
            None,
        )
        .unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
    }

    #[test]
    fn rejects_invalid_repo_name() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "invalid/repo",
            user_id,
            "johndoe",
            "user",
            "public",
            None,
            false,
            None,
            None,
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn rejects_invalid_owner_name() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "invalid@owner",
            "user",
            "public",
            None,
            false,
            None,
            None,
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn rejects_invalid_owner_type() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "my-repo", user_id, "johndoe", "invalid", "public", None, false, None, None,
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn rejects_invalid_visibility() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "my-repo", user_id, "johndoe", "user", "invalid", None, false, None, None,
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn trims_and_keeps_description() {
        let user_id = Uuid::new_v4();
        let request = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "johndoe",
            "user",
            "public",
            Some("  hello world  "),
            false,
            None,
            None,
        )
        .unwrap();

        assert_eq!(request.description.as_deref(), Some("hello world"));
    }

    #[test]
    fn empty_description_becomes_none() {
        let user_id = Uuid::new_v4();
        let request = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "johndoe",
            "user",
            "public",
            Some("   "),
            false,
            None,
            None,
        )
        .unwrap();

        assert_eq!(request.description, None);
    }

    #[test]
    fn accepts_valid_init_options() {
        let user_id = Uuid::new_v4();
        let request = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "johndoe",
            "user",
            "public",
            None,
            true,
            Some("rust"),
            Some("mit"),
        )
        .unwrap();

        assert!(request.init_readme);
        assert_eq!(request.gitignore, Some(GitignoreTemplate::Rust));
        assert_eq!(request.license, Some(LicenseTemplate::Mit));
    }

    #[test]
    fn rejects_invalid_gitignore_template() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "johndoe",
            "user",
            "public",
            None,
            false,
            Some("cobol"),
            None,
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }

    #[test]
    fn rejects_invalid_license_template() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new(
            "my-repo",
            user_id,
            "johndoe",
            "user",
            "public",
            None,
            false,
            None,
            Some("gpl-3.0"),
        );

        assert!(matches!(result, Err(RepositoryError::Input(_))));
    }
}
