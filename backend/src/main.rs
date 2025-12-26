use crate::config::settings::Settings;
use axum::{Router, routing::get};

mod config;

#[tokio::main]
async fn main() {
    let _settings = Settings::new().expect("Failed to load settings");
    println!("{}", _settings.database_url);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
