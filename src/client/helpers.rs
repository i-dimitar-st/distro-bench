use crate::models::Summary;
use log::{info, warn};
use serde_json::to_string_pretty;
use std::fs::read_to_string;
use std::fs::write;
use std::thread::sleep;
use std::time::Duration;

pub fn get_kernel_version(path: &str) -> String {
    read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn get_distro(path: &str) -> String {
    return read_to_string(path)
        .ok()
        .and_then(|each| {
            each.lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .map(|line| {
                    line.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string()
                })
        })
        .unwrap_or_else(|| "unknown".to_string());
}

pub fn sanitize_distro(input: &str) -> String {
    return input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
}

pub fn save_to_disk(filename: &str, summary: &Summary) -> () {
    let json = to_string_pretty(summary).expect("failed to serialize");
    for index in 5..10 {
        if write(filename, &json).is_ok() {
            return;
        }
        sleep(Duration::from_millis(index * 100));
    }
    warn!("failed to write");
    info!("Summary: {}", json);
}

pub fn build_filename(distro: &str) -> String {
    return format!("results/bench-[{}].json", sanitize_distro(distro));
}

pub fn percentile(sorted_data: &[u32], percentile: usize) -> u32 {
    if sorted_data.is_empty() {
        return 0;
    }
    let index = (sorted_data.len() * percentile / 100).min(sorted_data.len() - 1);
    return sorted_data[index];
}
