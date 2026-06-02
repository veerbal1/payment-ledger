use axum::{Router, http::StatusCode};
use tokio::net::TcpListener;

async fn home() -> (StatusCode, String) {
    (StatusCode::OK, "home".to_string())
}

async fn health() -> (StatusCode, String) {
    (StatusCode::OK, "working".to_string())
}

pub fn create_app() -> Router {
    use axum::routing::get;

    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = TcpListener::bind(address).await.unwrap();
    let app = create_app();
    axum::serve(listener, app).await.unwrap();
}
