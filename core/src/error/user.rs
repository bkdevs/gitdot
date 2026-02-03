use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid user name: {0}")]
    InvalidUserName(String),

    #[error("User not found: {0}")]
    NotFound(String),

    #[error("Name already taken: {0}")]
    NameTaken(String),

    #[error("Email already taken: {0}")]
    EmailTaken(String),

    #[error("Reserved name: {0}")]
    ReservedName(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Supabase error: {0}")]
    SupabaseError(#[from] supabase::Error),
}
