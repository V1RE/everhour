use crate::api::Api;
use crate::Result;

use humantime::format_duration;
use serde_json::Value;
use std::time::Duration;

fn get_current_timer() -> Result<Value> {
    Api::default().get("timers/current")
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn active_timer_value() -> Value {
        json!({
          "status": "active",
          "duration": 16,
          "today": 10_396,
        })
    }

    fn stopped_timer_value() -> Value {
        json!({"status": "stopped"})
    }

    #[test]
    fn timer_active() {
        assert!(is_timer_active(active_timer_value()));
        assert!(!is_timer_active(stopped_timer_value()));
    }

    #[test]
    fn timer_duration() {
        assert_eq!(get_timer_duration(active_timer_value()), "2h 53m 16s");
        assert_eq!(get_timer_duration(json!({"duration": 7200})), "2h");
        assert_eq!(get_timer_duration(stopped_timer_value()), "0s");
    }

    #[test]
    fn status_short() {
        assert_eq!(" 2h 53m 16s", get_status_short(active_timer_value()))
    }
}
