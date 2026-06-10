use uuid::Uuid;

use crate::{dto::OwnerName, error::OrganizationError};

#[derive(Debug, Clone)]
pub struct CreateOrganizationRequest {
    pub org_name: OwnerName,
    pub owner_id: Uuid,
    pub readme: Option<String>,
}

impl CreateOrganizationRequest {
    pub fn new(
        org_name: &str,
        owner_id: Uuid,
        readme: Option<String>,
    ) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::parse(org_name, "organization name")?,
            owner_id,
            readme,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("my-org", owner_id, None).unwrap();

        assert_eq!(request.org_name.as_ref(), "my-org");
        assert_eq!(request.owner_id, owner_id);
    }

    #[test]
    fn valid_with_numbers() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("org123", owner_id, None).unwrap();

        assert_eq!(request.org_name.as_ref(), "org123");
    }

    #[test]
    fn rejects_underscore() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("my_org", owner_id, None);

        assert!(matches!(result, Err(OrganizationError::Input(_))));
    }

    #[test]
    fn sanitizes_to_lowercase() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("MyOrg", owner_id, None).unwrap();

        assert_eq!(request.org_name.as_ref(), "myorg");
    }

    #[test]
    fn rejects_empty_name() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("", owner_id, None);

        assert!(matches!(result, Err(OrganizationError::Input(_))));
    }

    #[test]
    fn rejects_special_characters() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("my@org", owner_id, None);

        assert!(matches!(result, Err(OrganizationError::Input(_))));
    }

    #[test]
    fn rejects_starting_with_hyphen() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("-myorg", owner_id, None);

        assert!(matches!(result, Err(OrganizationError::Input(_))));
    }
}
