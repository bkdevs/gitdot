use api::user::UserEndpointResponse;
use gitdot_core::dto::UserResponse;

pub struct UserResponseWrapper(pub UserEndpointResponse);

impl From<UserResponse> for UserResponseWrapper {
    fn from(response: UserResponse) -> Self {
        UserResponseWrapper(UserEndpointResponse {
            id: response.id,
            name: response.name,
            email: response.email,
            created_at: response.created_at,
        })
    }
}
