use crate::config::CONCURRENCY;
use crate::config::TOTAL_REQUESTS;
use crate::config::{get_distro, get_kernel_version};
use crate::models::Summary;
use serde_json::to_string_pretty;
use std::fs::write;

pub fn save_to_disk(filename: &str, mut data: Vec<u128>) -> () {
    data.sort_unstable();
    let json = to_string_pretty(&Summary {
        distro: get_distro(),
        kernel: get_kernel_version(),
        total: *TOTAL_REQUESTS,
        concurrency: *CONCURRENCY,
        success: data.len() as u64,
        p50_ms: data[data.len() * 50 / 100] / 1000,
        p95_ms: data[data.len() * 95 / 100] / 1000,
        p99_ms: data[data.len() * 99 / 100] / 1000,
    })
    .expect("failed to serialize");
    write(filename, json).expect("failed to write file");
}
