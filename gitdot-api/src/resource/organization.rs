use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationResource {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMemberResource {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub organization_id: Uuid,
    pub org_name: String,
    pub role: String,
    pub role_description: Option<String>,
    pub created_at: DateTime<Utc>,
}
