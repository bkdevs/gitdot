use api::user::UserApiResponse;
use gitdot_core::dto::UserResponse;

pub struct UserResponseWrapper(pub UserApiResponse);

impl From<UserResponse> for UserResponseWrapper {
    fn from(response: UserResponse) -> Self {
        UserResponseWrapper(UserApiResponse {
            id: response.id,
            name: response.name,
            email: response.email,
            created_at: response.created_at,
        })
    }
}
