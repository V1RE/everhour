use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::error::Error;

pub fn get(path: String) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get(format!(
            "https://private-anon-8eb29346f3-everhour.apiary-mock.com/{}",
            path
        ))
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", "aaaa-bbbb-cccddd-eeefff-gggghhii")
        .send()?
        .json::<serde_json::Value>()?;

    Ok(serde_json::json!(res))
}
