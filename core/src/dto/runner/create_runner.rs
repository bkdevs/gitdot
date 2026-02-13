use uuid::Uuid;

use crate::dto::{OwnerName, common::RunnerName};
use crate::error::RunnerError;
use crate::model::RunnerOwnerType;

#[derive(Debug, Clone)]
pub struct CreateRunnerRequest {
    pub name: RunnerName,
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RunnerOwnerType,
}

impl CreateRunnerRequest {
    pub fn new(
        name: &str,
        user_id: Uuid,
        owner_name: &str,
        owner_type: &str,
    ) -> Result<Self, RunnerError> {
        Ok(Self {
            name: RunnerName::try_new(name)
                .map_err(|e| RunnerError::InvalidRunnerName(e.to_string()))?,
            user_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RunnerError::InvalidOwnerName(e.to_string()))?,
            owner_type: owner_type.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let user_id = Uuid::new_v4();
        let request = CreateRunnerRequest::new("my-runner", user_id, "johndoe", "user").unwrap();

        assert_eq!(request.name.as_ref(), "my-runner");
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.owner_name.as_ref(), "johndoe");
        assert_eq!(request.owner_type, RunnerOwnerType::User);
    }

    #[test]
    fn valid_organization_runner() {
        let user_id = Uuid::new_v4();
        let request = CreateRunnerRequest::new("runner", user_id, "myorg", "organization").unwrap();

        assert_eq!(request.owner_type, RunnerOwnerType::Organization);
    }

    #[test]
    fn rejects_invalid_runner_name() {
        let user_id = Uuid::new_v4();
        let result = CreateRunnerRequest::new("invalid/runner", user_id, "johndoe", "user");

        assert!(matches!(result, Err(RunnerError::InvalidRunnerName(_))));
    }

    #[test]
    fn rejects_empty_runner_name() {
        let user_id = Uuid::new_v4();
        let result = CreateRunnerRequest::new("", user_id, "johndoe", "user");

        assert!(matches!(result, Err(RunnerError::InvalidRunnerName(_))));
    }

    #[test]
    fn rejects_invalid_owner_name() {
        let user_id = Uuid::new_v4();
        let result = CreateRunnerRequest::new("my-runner", user_id, "invalid@owner", "user");

        assert!(matches!(result, Err(RunnerError::InvalidOwnerName(_))));
    }

    #[test]
    fn rejects_invalid_owner_type() {
        let user_id = Uuid::new_v4();
        let result = CreateRunnerRequest::new("my-runner", user_id, "johndoe", "invalid");

        assert!(matches!(result, Err(RunnerError::InvalidOwnerType(_))));
    }
}
