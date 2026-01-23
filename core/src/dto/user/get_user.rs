use super::UserName;

#[derive(Debug, Clone)]
pub struct GetUserRequest {
    pub user_name: UserName,
}

impl GetUserRequest {
    pub fn new(user_name: &str) -> Self {
        Self {
            user_name: UserName::try_new(user_name).unwrap(),
        }
    }
}
