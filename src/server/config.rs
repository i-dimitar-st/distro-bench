use once_cell::sync::Lazy;
use std::{
    env::var,
    net::{IpAddr, SocketAddr},
};

pub static HOST: Lazy<IpAddr> = Lazy::new(|| {
    var("HOST")
        .ok()
        .as_deref()
        .unwrap_or("127.0.0.1")
        .parse()
        .expect("valid IP address")
});

pub static PORT: Lazy<u16> = Lazy::new(|| {
    var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .filter(|port| (1024..65000).contains(port))
        .unwrap_or(55555)
});

pub static ADDR: Lazy<SocketAddr> = Lazy::new(|| SocketAddr::new(*HOST, *PORT));
