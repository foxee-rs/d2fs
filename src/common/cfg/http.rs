use reqwest::Client;
use std::{sync::OnceLock, time::Duration};

pub fn singleton_http_client() -> Client {
    static INSTANCE: OnceLock<Client> = OnceLock::new();
    let c = INSTANCE.get_or_init(|| {
        Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap()
    });
    c.clone()
}
