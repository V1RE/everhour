use crate::{config, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use url::Url;

type Json = Result<Value>;

pub struct Api {
    client: Client,
    base_url: Url,
}

// ^([a-z0-9]+-?)+$

impl Default for Api {
    fn default() -> Self {
        Self::new(config::API_KEY, config::BASE_URL)
    }
}

impl Api {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        let api_header_value = HeaderValue::from_str(api_key).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", api_header_value);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let timeout = std::time::Duration::from_secs(10);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .expect("The TLS backend cannot be initialized or the resolver cannot load the system configuration.");

        let base_url = Url::parse(base_url).expect("Invalid base URL");

        Self { client, base_url }
    }

    pub fn url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    pub fn get(&self, path: &str) -> Json {
        let res = &self
            .client
            .get(self.url(path))
            .send()?
            .json::<serde_json::Value>()?;

        Ok(serde_json::json!(res))
    }
}
