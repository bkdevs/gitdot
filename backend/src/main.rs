use axum::{
    Router,
    routing::{get, post},
};
use config::settings::Settings;

mod config;
mod git;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load settings");
    println!("DATABASE_URL={}", settings.database_url);
    println!("GIT_PROJECT_ROOT={}", settings.git_project_root);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/{owner}/{repo}/info/refs",
            get(git::smart_http::git_info_refs),
        )
        .route(
            "/{owner}/{repo}/git-upload-pack",
            post(git::smart_http::git_upload_pack),
        )
        .route(
            "/{owner}/{repo}/git-receive-pack",
            post(git::smart_http::git_receive_pack),
        )
        .with_state(settings.clone());

    let address = format!("{}:{}", settings.server_host, settings.server_port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
