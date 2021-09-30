use reqwest::blocking::Client;
use reqwest::header;
use serde_json::Value;
use std::error::Error;

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Api-Key",
            header::HeaderValue::from_static("aaaa-bbbb-cccddd-eeefff-gggghhii"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let timeout = std::time::Duration::from_secs(10);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .unwrap();

        Self { client }
    }

    pub fn get(&self, path: &str) -> Result<Value, Box<dyn Error>> {
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
