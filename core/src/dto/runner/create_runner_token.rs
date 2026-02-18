use uuid::Uuid;

pub struct CreateRunnerTokenRequest {
    pub runner_id: Uuid,
}

pub struct CreateRunnerTokenResponse {
    pub token: String,
}
