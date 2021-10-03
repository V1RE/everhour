use crate::{config, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::{json, Value};
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
        let res = &self.client.get(self.url(path)).send()?.json::<Value>()?;

        Ok(json!(res))
    }

    pub fn delete(&self, path: &str) -> Json {
        let res = &self.client.delete(self.url(path)).send()?.json::<Value>()?;

        Ok(json!(res))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::timer::tests::active_timer_value;
    use httpmock::prelude::*;

    pub fn server_api() -> (MockServer, Api) {
        let server = MockServer::start();
        let api = Api::new(config::API_KEY, &server.base_url());
        (server, api)
    }

    #[test]
    fn test_get() {
        // Start a lightweight mock server.
        let (server, api) = server_api();

        // Create a mock on the server.
        let mock_get_current = server.mock(|when, then| {
            when.method(GET).path("/timers/current");
            then.status(200).body(active_timer_value().to_string());
        });

        let res = api.get("/timers/current");

        mock_get_current.assert();
        assert!(&res.is_ok());
        assert_eq!(&res.unwrap(), &active_timer_value());
    }
}
