use crate::models::NetworkIdentifier;
use reqwest::header::HeaderMap;
use std::time::Duration;

/// How [`crate::RosettaClient`] reaches a server.
///
/// A request is tried `retries` times in all, `retry_delay` apart, when the
/// server cannot be reached, answers HTTP 5xx without a Rosetta error, or
/// returns a Rosetta error marked `retriable`.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// The server's base URL, such as `http://127.0.0.1:3087`.
    pub base_url: String,
    /// The network every request is for.
    pub network: NetworkIdentifier,
    /// Attempts per request, at least 1.
    pub retries: u32,
    /// The wait between attempts.
    pub retry_delay: Duration,
    /// The timeout of one attempt, more than zero.
    pub timeout: Duration,
    /// Headers for every request, such as an API key.
    pub headers: HeaderMap,
}

impl Default for ClientConfig {
    /// Mainnet on the default local port: 3 attempts 5 s apart, 30 s each.
    fn default() -> Self {
        ClientConfig {
            base_url: format!("http://127.0.0.1:{}", crate::DEFAULT_PORT),
            network: NetworkIdentifier::mina(crate::MAINNET),
            retries: 3,
            retry_delay: Duration::from_secs(5),
            timeout: Duration::from_secs(30),
            headers: HeaderMap::new(),
        }
    }
}
