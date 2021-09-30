use humantime::format_duration;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;
use std::process;
use std::time::Duration;

fn timer() -> Result<bool, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .get("https://api.everhour.com/timers/current")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Api-Key", "asdf")
        .send()?
        .json::<serde_json::Value>()?;

    let timer = serde_json::json!(res);

    if timer["duration"].is_number() {
        let duration = Duration::new(timer["duration"].as_u64().unwrap(), 0);
        let time = format_duration(duration).to_string();
        println!(" {}", time);
        return Ok(true);
    }

    Ok(false)
}

fn main() {
    let result = timer();

    match result {
        Err(_) => {
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
