use serde::Serialize;

#[derive(Serialize)]
pub struct Summary {
    pub distro: String,
    pub kernel: String,
    pub total: usize,
    pub concurrency: usize,
    pub success: usize,
    pub p50: u32,
    pub p90: u32,
    pub p99: u32,
}
