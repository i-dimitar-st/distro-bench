use once_cell::sync::Lazy;
use std::env::var;
use std::fs::read_to_string;
use std::net::SocketAddr;

pub static HOST: Lazy<String> =
    Lazy::new(|| var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()));

pub static PORT: Lazy<u16> = Lazy::new(|| {
    var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(55555)
});

pub static ADDR: Lazy<SocketAddr> = Lazy::new(|| {
    format!("{}:{}", *HOST, *PORT)
        .parse()
        .expect("valid socket address")
});

pub static TOTAL_REQUESTS: Lazy<u64> = Lazy::new(|| {
    var("TOTAL_REQUESTS")
        .unwrap_or_else(|_| "500000".to_string())
        .parse()
        .unwrap()
});

pub static CONCURRENCY: Lazy<u64> = Lazy::new(|| {
    var("CONCURRENCY")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .unwrap()
});

const OS_RELEASE_PATH: &str = "/etc/os-release";
const KERNEL_RELEASE_PATH: &str = "/proc/sys/kernel/osrelease";

pub fn get_kernel_version() -> String {
    read_to_string(KERNEL_RELEASE_PATH)
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub fn get_distro() -> String {
    read_to_string(OS_RELEASE_PATH)
        .ok()
        .and_then(|s| {
            s.lines().find(|l| l.starts_with("PRETTY_NAME=")).map(|l| {
                l.trim_start_matches("PRETTY_NAME=")
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
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}
