use env_logger::Builder;
use env_logger::Env;
use log::info;
use std::time::Instant;

mod benchmark;
mod config;
mod helpers;
mod models;

use crate::helpers::{build_filename, get_distro, get_kernel_version, percentile, save_to_disk};
use benchmark::run_benchmark;
use config::{CONCURRENCY, KERNEL_RELEASE_PATH, OS_RELEASE_PATH, TOTAL_REQUESTS};
use models::Summary;

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting benchmark");
    let start = Instant::now();

    let results = run_benchmark(*TOTAL_REQUESTS, *CONCURRENCY).await;
    let mut sorted_results = results.clone();
    sorted_results.sort_unstable();

    let distro = get_distro(OS_RELEASE_PATH);
    let kernel = get_kernel_version(KERNEL_RELEASE_PATH);
    let success = results.len();
    let p50 = percentile(&sorted_results, 50);
    let p90 = percentile(&sorted_results, 90);
    let p99 = percentile(&sorted_results, 99);

    let filename = build_filename(&distro);
    let summary = Summary {
        distro: distro,
        kernel: kernel,
        duration: start.elapsed().as_secs() as u32,
        total: *TOTAL_REQUESTS,
        concurrency: *CONCURRENCY,
        success: success,
        p50: p50,
        p90: p90,
        p99: p99,
    };
    save_to_disk(&filename, &summary);

    info!("Benchmark complete in {} sec", start.elapsed().as_secs());
}
