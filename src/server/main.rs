use axum::{Router, response::IntoResponse, routing::get, serve};
use log::info;
use once_cell::sync::Lazy;
use std::fs::read;
use tokio::net::TcpListener;

mod config;
use config::ADDR;
use env_logger::{Builder, Env};

static PAYLOAD: Lazy<Vec<u8>> = Lazy::new(|| read("payload").expect("failed"));

async fn http_handler() -> impl IntoResponse {
    (
        [("Content-Type", "application/octet-stream")],
        PAYLOAD.clone(),
    )
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
