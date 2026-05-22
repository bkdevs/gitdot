use thiserror::Error;

use crate::error::{ClickHouseError, InputError};

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    ClickHouse(#[from] ClickHouseError),
}
