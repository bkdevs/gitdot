use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommentServerRequest {
    pub body: String,
}
