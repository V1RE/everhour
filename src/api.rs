use crate::{config, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;

type Json = Result<Value>;

pub struct Api {
    client: Client,
}

// ^([a-z0-9]+-?)+$

impl Default for Api {
    fn default() -> Self {
        Self::new(config::API_KEY)
    }
}

impl Api {
    pub fn new(api_key: &str) -> Self {
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

        Self { client }
    }

    pub fn get(&self, path: &str) -> Json {
        let res = &self
            .client
            .get(format!(
                "https://private-anon-8eb29346f3-everhour.apiary-mock.com/{}",
                path
            ))
            .send()?
            .json::<serde_json::Value>()?;

        Ok(serde_json::json!(res))
    }
}
