mod config;
mod dto;
mod handlers;
mod utils;

use axum::{
    Router,
    routing::{get, post},
};
use config::settings::Settings;
use handlers::git_smart_http::{git_info_refs, git_receive_pack, git_upload_pack};
use handlers::repository::{create_repository, get_repository_file, get_repository_tree};
use std::sync::Arc;

fn create_router(settings: Arc<Settings>) -> Router {
    let git_router = Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route("/{owner}/{repo}/git-upload-pack", post(git_upload_pack))
        .route("/{owner}/{repo}/git-receive-pack", post(git_receive_pack));

    let repo_router = Router::new()
        .route("/repository/{owner}/{repo}", post(create_repository))
        .route("/repository/{owner}/{repo}/tree", get(get_repository_tree))
        .route("/repository/{owner}/{repo}/file", get(get_repository_file));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(git_router)
        .merge(repo_router)
        .with_state(settings)
}

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load settings");
    let address = settings.get_server_address();
    let app = create_router(settings);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
