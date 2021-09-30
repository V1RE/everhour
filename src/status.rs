use crate::api::Api;
use humantime::format_duration;
use std::error::Error;
use std::time::Duration;

pub fn status(api: Api) -> Result<bool, Box<dyn Error>> {
    let timer = match api.get("timers/current") {
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
