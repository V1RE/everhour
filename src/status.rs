use humantime::format_duration;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

fn get_timer() -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get("https://private-anon-8eb29346f3-everhour.apiary-mock.com/timers/current")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", "aaaa-bbbb-cccddd-eeefff-gggghhii")
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
        let mut timer_duration = timer["duration"].as_u64().unwrap();
        timer_duration = timer["today"].as_u64().unwrap_or(timer_duration);

        let duration = Duration::new(timer_duration, 0);
        let time = format_duration(duration).to_string();

        println!(" {}", time);

        Ok(true)
    } else {
        Ok(false)
    }
}
