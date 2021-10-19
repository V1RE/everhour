use crate::api::API;
use crate::Result;
use humantime::format_duration;
use serde_json::Value;
use std::time::Duration;

pub fn is_timer_active(timer: Value) -> bool {
    timer["duration"].is_number()
}

pub fn get_timer_duration(timer: Value) -> String {
    let timer_duration = timer["duration"].as_u64().unwrap_or(0);
    let timer_today = timer["today"].as_u64().unwrap_or(0);
    let timer_total = timer_duration + timer_today;

    let duration = Duration::from_secs(timer_total);
    format_duration(duration).to_string()
}

pub fn get_timer_name(timer: Value) -> String {
    timer["task"]["name"].as_str().unwrap_or("").to_string()
}

pub fn stop() -> Result<bool> {
    match API.delete("/timers/current") {
        Ok(_) => Ok(true),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{json, Value};

    pub fn active_timer_value() -> Value {
        json!({
          "status": "active",
          "duration": 16,
          "today": 10_396,
          "task": {
              "name": "active timer"
            }
        })
    }

    pub fn stopped_timer_value() -> Value {
        json!({"status": "stopped"})
    }

    #[test]
    fn timer_active() {
        assert!(is_timer_active(active_timer_value()));
        assert!(!is_timer_active(stopped_timer_value()));
    }

    #[test]
    fn timer_duration() {
        assert_eq!(get_timer_duration(active_timer_value()), "2h 53m 32s");
        assert_eq!(get_timer_duration(json!({"duration": 7200})), "2h");
        assert_eq!(get_timer_duration(stopped_timer_value()), "0s");
    }

    #[test]
    fn timer_name() {
        assert_eq!(get_timer_name(active_timer_value()), "active timer");
        assert_eq!(get_timer_name(stopped_timer_value()), "");
    }
}
