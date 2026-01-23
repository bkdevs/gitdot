#[derive(Debug, Clone)]
pub struct InfoRefsRequest {
    pub owner: String,
    pub repo: String,
    pub service: String,
}
