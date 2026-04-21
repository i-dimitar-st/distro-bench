use crate::models::Summary;
use serde_json::to_string_pretty;
use std::fs::read_to_string;
use std::fs::write;

pub fn get_kernel_version(path: &str) -> String {
    read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn get_distro(path: &str) -> String {
    read_to_string(path)
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
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn sanitize_distro(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|each_char| {
            if each_char.is_whitespace() {
                '-'
            } else {
                each_char
            }
        })
        .filter(|each_char| each_char.is_ascii_alphanumeric() || *each_char == '-')
        .collect()
}

pub fn save_to_disk(filename: &str, summary: &Summary) {
    let json = to_string_pretty(summary).expect("failed to serialize");
    write(filename, json).expect("failed to write file");
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
