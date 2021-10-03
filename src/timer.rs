use crate::api::API;
use crate::Result;
use humantime::format_duration;
use serde_json::Value;
use std::time::Duration;

pub fn is_timer_active(timer: Value) -> bool {
    timer["duration"].is_number()
}

pub fn get_timer_duration(timer: Value) -> String {
    let mut timer_duration = timer["duration"].as_u64().unwrap_or(0);
    timer_duration = timer["today"].as_u64().unwrap_or(timer_duration);

    let duration = Duration::from_secs(timer_duration);
    format_duration(duration).to_string()
}

pub fn stop() -> Result<bool> {
    let timer = match API.delete("/timers/current") {
        Ok(timer) => timer,
        Err(err) => return Err(err),
    };
    println!("{:?}", is_timer_active(timer));
    Ok(true)
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
        assert_eq!(get_timer_duration(active_timer_value()), "2h 53m 16s");
        assert_eq!(get_timer_duration(json!({"duration": 7200})), "2h");
        assert_eq!(get_timer_duration(stopped_timer_value()), "0s");
    }
}
