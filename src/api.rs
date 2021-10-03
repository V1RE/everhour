use crate::{config, Result};
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::{json, Value};
use url::Url;

type Json = Result<Value>;

lazy_static! {
    pub static ref API: Api = Api::default();
}

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
    fn new(api_key: &str, base_url: &str) -> Self {
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
    use crate::{config::BASE_URL, timer::tests::active_timer_value};
    use httpmock::prelude::*;

    pub fn server_api() -> (MockServer, Api) {
        let server = MockServer::start();
        let api = Api::new(config::API_KEY, &server.base_url());
        (server, api)
    }

    #[test]
    fn test_url() {
        assert_eq!(BASE_URL.to_string(), API.url("/").to_string())
    }

    #[ignore]
    #[test]
    fn test_timeout() {
        // Start a lightweight mock server.
        let (server, api) = server_api();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET).path("/3");
            then.status(200)
                .delay(std::time::Duration::from_secs(3))
                .json_body(active_timer_value());
        });

        let err_mock_timeout = server.mock(|when, then| {
            when.method(GET).path("/11");
            then.status(200)
                .delay(std::time::Duration::from_secs(11))
                .json_body(active_timer_value());
        });

        let res = api.get("/3");
        let res_err = api.get("/11");

        mock.assert();
        err_mock_timeout.assert();
        assert!(&res.is_ok());
        assert!(&res_err.is_err());
    }

    #[test]
    fn test_get() {
        // Start a lightweight mock server.
        let (server, api) = server_api();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get");
            then.status(200).json_body(active_timer_value());
        });

        let err_mock_json = server.mock(|when, then| {
            when.method(GET).path("/json");
            then.status(200).body("Non-JSON data");
        });

        let res = api.get("/get");
        let res_err = api.get("/json");

        mock.assert();
        err_mock_json.assert();
        assert!(&res.is_ok());
        assert_eq!(&res.unwrap(), &active_timer_value());
        assert!(&res_err.is_err());
    }

    #[test]
    fn test_delete() {
        // Start a lightweight mock server.
        let (server, api) = server_api();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(DELETE).path("/delete");
            then.status(200).json_body(active_timer_value());
        });

        let err_mock_json = server.mock(|when, then| {
            when.method(DELETE).path("/json");
            then.status(200).body("Non-JSON data");
        });

        let res = api.delete("/delete");
        let res_err = api.delete("/json");

        mock.assert();
        err_mock_json.assert();
        assert!(&res.is_ok());
        assert_eq!(&res.unwrap(), &active_timer_value());
        assert!(&res_err.is_err());
    }
}
