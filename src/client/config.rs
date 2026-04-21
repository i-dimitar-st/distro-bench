use once_cell::sync::Lazy;
use std::env::var;
use std::net::SocketAddr;

pub static HOST: Lazy<String> =
    Lazy::new(|| var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()));

pub static PORT: Lazy<usize> = Lazy::new(|| {
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

pub static URL: Lazy<String> = Lazy::new(|| format!("http://{}", *ADDR));

pub static TOTAL_REQUESTS: Lazy<usize> = Lazy::new(|| {
    var("TOTAL_REQUESTS")
        .unwrap_or_else(|_| "500000".to_string())
        .parse()
        .unwrap()
});

pub static CONCURRENCY: Lazy<usize> = Lazy::new(|| {
    var("CONCURRENCY")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .unwrap()
});

pub static OS_RELEASE_PATH: &str = "/etc/os-release";

pub static KERNEL_RELEASE_PATH: &str = "/proc/sys/kernel/osrelease";
