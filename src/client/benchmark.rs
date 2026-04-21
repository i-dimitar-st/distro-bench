use crate::config::URL;
pub use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Instant;
use tokio::task::JoinSet;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn run_request(url: &str) -> Option<u32> {
    let start = Instant::now();
    let res = CLIENT.get(url).send().await.ok()?;
    res.bytes().await.ok()?; // This to simulate result work
    return Some(start.elapsed().as_millis() as u32);
}

pub async fn run_benchmark(total_req: usize, max_concurrent_req: usize) -> Vec<u32> {
    let mut set = JoinSet::new();
    let mut results = Vec::with_capacity(total_req);
    let mut started = 0;

    // initial
    while started < total_req && set.len() < max_concurrent_req {
        set.spawn(async { run_request(&URL).await });
        started += 1;
    }

    // main
    while let Some(res) = set.join_next().await {
        if let Ok(Some(value)) = res {
            results.push(value);
        }

        // fill if required
        if started < total_req {
            set.spawn(async { run_request(&URL).await });
            started += 1;
        }
    }
    return results;
}
