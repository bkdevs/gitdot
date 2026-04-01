use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid {field}: {reason}")]
pub struct InputError {
    pub field: &'static str,
    pub reason: String,
}

impl InputError {
    pub fn new(field: &'static str, reason: impl ToString) -> Self {
        Self {
            field,
            reason: reason.to_string(),
        }
    }
}
