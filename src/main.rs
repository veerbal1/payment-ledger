use axum::{Router, http::StatusCode};
use sqlx::{Postgres, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use payment_ledger::handlers::create_transfer;
use payment_ledger::state::AppState;

async fn home() -> (StatusCode, String) {
    (StatusCode::OK, "home".to_string())
}

async fn health() -> (StatusCode, String) {
    (StatusCode::OK, "working".to_string())
}

pub fn create_app(pool: sqlx::Pool<Postgres>) -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route("/v1", get(home))
        .route("/v1/health", get(health))
        .route("/v1/transfers", post(create_transfer))
        .with_state(AppState { pool })
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&database_url)
        .await
        .expect("failed to create database pool");

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = TcpListener::bind(address).await.unwrap();
    let app = create_app(pool);
    axum::serve(listener, app).await.unwrap();
}
