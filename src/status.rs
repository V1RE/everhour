use humantime::format_duration;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

fn get_timer() -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get("https://api.everhour.com/timers/current")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", "asdf")
        .send()?
        .json::<serde_json::Value>()?;

    Ok(serde_json::json!(res))
}

pub fn status() -> Result<bool, Box<dyn Error>> {
    let timer = match get_timer() {
        Ok(timer) => timer,
        Err(err) => return Err(err),
    };

    if timer["duration"].is_number() {
        let duration = Duration::new(timer["duration"].as_u64().unwrap(), 0);
        let time = format_duration(duration).to_string();
        println!(" {}", time);
        return Ok(true);
    }

    Ok(false)
}
