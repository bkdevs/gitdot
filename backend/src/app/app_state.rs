use std::sync::Arc;

use axum::extract::FromRef;

use super::settings::Settings;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    settings: Arc<Settings>,
}

impl AppState {
    pub fn new(settings: Arc<Settings>) -> Self {
        Self { settings }
    }
}
