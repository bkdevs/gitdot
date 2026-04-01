use thiserror::Error;

#[derive(Debug, Error)]
#[error("{entity} not found: {id}")]
pub struct NotFoundError {
    pub entity: &'static str,
    pub id: String,
}

impl NotFoundError {
    pub fn new(entity: &'static str, id: impl ToString) -> Self {
        Self {
            entity,
            id: id.to_string(),
        }
    }
}
