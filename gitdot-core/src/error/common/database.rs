use thiserror::Error;

use super::NotFoundError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Row not found")]
    RowNotFound,

    #[error("Database error: {0}")]
    Other(sqlx::Error),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Self::RowNotFound,
            other => Self::Other(other),
        }
    }
}

pub trait NotFoundExt<T> {
    fn or_not_found<E: From<NotFoundError> + From<DatabaseError>>(
        self,
        entity: &'static str,
        id: impl ToString,
    ) -> Result<T, E>;
}

impl<T> NotFoundExt<T> for Result<T, DatabaseError> {
    fn or_not_found<E: From<NotFoundError> + From<DatabaseError>>(
        self,
        entity: &'static str,
        id: impl ToString,
    ) -> Result<T, E> {
        match self {
            Ok(v) => Ok(v),
            Err(DatabaseError::RowNotFound) => Err(NotFoundError::new(entity, id).into()),
            Err(e) => Err(e.into()),
        }
    }
}
