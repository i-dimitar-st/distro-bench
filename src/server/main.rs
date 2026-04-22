use axum::{Router, response::IntoResponse, routing::get, serve};
use log::info;
use once_cell::sync::Lazy;
use serde_json::{Value, from_str, to_vec};
use std::fs::read_to_string;
use tokio::net::TcpListener;

mod config;
use config::ADDR;
use env_logger::{Builder, Env};

static PAYLOAD: Lazy<String> =
    Lazy::new(|| read_to_string("payload.json").expect("failed to read file"));

async fn http_handler() -> impl IntoResponse {
    // Parsed/stringiifed on every request
    let json: Value = from_str(&PAYLOAD).expect("invalid json");
    let http_body = to_vec(&json).expect("serialization failed");
    ([("Content-Type", "application/json")], http_body)
}

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    let app = Router::new().route("/", get(http_handler));
    let listener = TcpListener::bind(*ADDR)
        .await
        .expect("failed to bind address");

    info!("server started on {}", *ADDR);
    serve(listener, app).await.unwrap();
}
