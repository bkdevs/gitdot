use uuid::Uuid;

use crate::dto::OwnerName;
use crate::error::OrganizationError;

#[derive(Debug, Clone)]
pub struct CreateOrganizationRequest {
    pub org_name: OwnerName,
    pub owner_id: Uuid,
}

impl CreateOrganizationRequest {
    pub fn new(org_name: &str, owner_id: Uuid) -> Result<Self, OrganizationError> {
        Ok(Self {
            org_name: OwnerName::try_new(org_name)
                .map_err(|e| OrganizationError::InvalidOrganizationName(e.to_string()))?,
            owner_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("my-org", owner_id).unwrap();

        assert_eq!(request.org_name.as_ref(), "my-org");
        assert_eq!(request.owner_id, owner_id);
    }

    #[test]
    fn valid_with_numbers() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("org123", owner_id).unwrap();

        assert_eq!(request.org_name.as_ref(), "org123");
    }

    #[test]
    fn valid_with_underscore() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("my_org", owner_id).unwrap();

        assert_eq!(request.org_name.as_ref(), "my_org");
    }

    #[test]
    fn sanitizes_to_lowercase() {
        let owner_id = Uuid::new_v4();
        let request = CreateOrganizationRequest::new("MyOrg", owner_id).unwrap();

        assert_eq!(request.org_name.as_ref(), "myorg");
    }

    #[test]
    fn rejects_empty_name() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("", owner_id);

        assert!(matches!(
            result,
            Err(OrganizationError::InvalidOrganizationName(_))
        ));
    }

    #[test]
    fn rejects_special_characters() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("my@org", owner_id);

        assert!(matches!(
            result,
            Err(OrganizationError::InvalidOrganizationName(_))
        ));
    }

    #[test]
    fn rejects_starting_with_hyphen() {
        let owner_id = Uuid::new_v4();
        let result = CreateOrganizationRequest::new("-myorg", owner_id);

        assert!(matches!(
            result,
            Err(OrganizationError::InvalidOrganizationName(_))
        ));
    }
}
