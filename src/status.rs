use crate::api::Api;
use crate::Result;

use humantime::format_duration;
use serde_json::Value;
use std::time::Duration;

fn get_current_timer() -> Result<Value> {
    Api::new().get("timers/current")
}

fn is_timer_active(timer: Value) -> bool {
    timer["duration"].is_number()
}

fn get_timer_duration(timer: Value) -> String {
    let mut timer_duration = timer["duration"].as_u64().unwrap_or(0);
    timer_duration = timer["today"].as_u64().unwrap_or(timer_duration);

    let duration = Duration::from_secs(timer_duration);
    format_duration(duration).to_string()
}

fn get_status_short(timer: Value) -> String {
    format!(" {}", get_timer_duration(timer))
}

pub fn status(long: bool) -> Result<bool> {
    let timer = match get_current_timer() {
        Ok(timer) => timer,
        Err(err) => return Err(err),
    };

    if is_timer_active(timer.clone()) {
        // TODO: Add status_long including task name
        let status = get_status_short(timer);
        println!("{}", status);

        Ok(true)
    } else {
        Ok(long)
    }
}
