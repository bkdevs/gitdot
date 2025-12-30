use axum::{Router, routing::get};
use config::settings::Settings;

mod config;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load settings");
    println!("DB URL: {}", settings.database_url);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let address = format!("{}:{}", settings.server_host, settings.server_port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
