use crate::api::API;
use crate::timer::{get_timer_duration, is_timer_active};
use crate::Result;
use serde_json::Value;

pub fn get_current_timer() -> Result<Value> {
    API.get("timers/current")
}

fn get_status_short(timer: Value) -> String {
    format!(" {}", get_timer_duration(timer))
}

pub fn short() -> Result<bool> {
    let timer = match get_current_timer() {
        Ok(timer) => timer,
        Err(err) => return Err(err),
    };

    if is_timer_active(timer.clone()) {
        let status = get_status_short(timer);
        println!("{}", status);

        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::get_status_short;
    use crate::timer::tests::*;

    #[test]
    fn status_short() {
        assert_eq!(" 2h 53m 16s", get_status_short(active_timer_value()));
        assert_eq!(" 0s", get_status_short(stopped_timer_value()));
    }
}
