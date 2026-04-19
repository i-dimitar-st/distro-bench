use serde::Serialize;

#[derive(Serialize)]
pub struct Summary {
    pub distro: String,
    pub kernel: String,
    pub total: u64,
    pub concurrency: u64,
    pub success: u64,
    pub p50_ms: u128,
    pub p95_ms: u128,
    pub p99_ms: u128,
}
