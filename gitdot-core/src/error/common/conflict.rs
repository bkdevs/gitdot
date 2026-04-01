use thiserror::Error;

#[derive(Debug, Error)]
#[error("{entity} already exists: {id}")]
pub struct ConflictError {
    pub entity: &'static str,
    pub id: String,
}

impl ConflictError {
    pub fn new(entity: &'static str, id: impl ToString) -> Self {
        Self {
            entity,
            id: id.to_string(),
        }
    }
}
