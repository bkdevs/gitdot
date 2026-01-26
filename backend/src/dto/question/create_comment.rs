use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentServerRequest {
    pub body: String,
}
