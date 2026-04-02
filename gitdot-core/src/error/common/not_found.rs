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

pub trait OptionNotFoundExt<T> {
    fn or_not_found(self, entity: &'static str, id: impl ToString) -> Result<T, NotFoundError>;
}

impl<T> OptionNotFoundExt<T> for Option<T> {
    fn or_not_found(self, entity: &'static str, id: impl ToString) -> Result<T, NotFoundError> {
        self.ok_or_else(|| NotFoundError::new(entity, id))
    }
}
