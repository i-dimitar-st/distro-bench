use axum::{Router, response::IntoResponse, routing::get, serve};
use log::info;
use once_cell::sync::Lazy;
use serde_json::{Value, from_str, to_vec};
use std::fs::read_to_string;
use tokio::net::TcpListener;

mod config;
use config::ADDR;
use env_logger::{Builder, Env};

// Leave this for cached value manips
static PAYLOAD: Lazy<Value> = Lazy::new(|| {
    let data = read_to_string("payload.json").expect("failed to read file");
    from_str(&data).expect("invalid json")
});

async fn http_handler_cached() -> impl IntoResponse {
    let http_body = to_vec(&*PAYLOAD).expect("serialization failed");
    ([("Content-Type", "application/json")], http_body)
}

// async fn http_handler_not_cached() -> impl IntoResponse {
//     let payload_str = read_to_string("payload.json").expect("failed to read file");
//     let payload: Value = from_str(&payload_str).expect("invalid json");
//     let http_body = to_vec(&payload).expect("serialization failed");
//     return ([("Content-Type", "application/json")], http_body);
// }

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let app = Router::new().route("/", get(http_handler_cached));
    let listener = TcpListener::bind(*ADDR)
        .await
        .expect("failed to bind address");

    info!("server started on {}", *ADDR);
    serve(listener, app).await.unwrap();
}
