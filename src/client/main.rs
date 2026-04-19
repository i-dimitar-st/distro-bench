use env_logger::Builder;
use env_logger::Env;
use log::info;
use std::time::Instant;

mod benchmark;
mod config;
mod models;
mod stats;

use crate::config::{get_distro, sanitize_distro};
use benchmark::run_benchmark;
use config::CONCURRENCY;
use config::TOTAL_REQUESTS;
use stats::save_to_disk;

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting benchmark");
    let start = Instant::now();

    let results = run_benchmark(*TOTAL_REQUESTS, *CONCURRENCY).await;
    save_to_disk(
        &format!("results/bench-[{}].json", sanitize_distro(&get_distro())),
        results,
    );

    let elapsed = start.elapsed();
    info!("Benchmark complete in {} sec", elapsed.as_secs());
}
