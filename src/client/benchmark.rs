use crate::config::ADDR;
pub use once_cell::sync::Lazy;
use reqwest::Client;
use tokio::task::JoinSet;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
pub static URL: Lazy<String> = Lazy::new(|| format!("http://{}", *ADDR));

pub async fn run_request(url: &str) -> Result<u128, ()> {
    let start = std::time::Instant::now();
    let res = CLIENT.get(url).send().await.map_err(|_| ())?;
    let _ = res.bytes().await.map_err(|_| ())?;
    Ok(start.elapsed().as_micros())
}

pub async fn run_benchmark(total: u64, concurrency: u64) -> Vec<u128> {
    let mut set = JoinSet::new();
    let mut results = Vec::with_capacity(total as usize);
    let mut sent = 0;

    // initial fill
    for _ in 0..concurrency.min(total) {
        set.spawn(async move { run_request(&URL).await });
        sent += 1;
    }

    while let Some(res) = set.join_next().await {
        if let Ok(Ok(latency)) = res {
            results.push(latency);
        }

        if sent < total {
            set.spawn(async move { run_request(&URL).await });
            sent += 1;
        }
    }

    results
}
