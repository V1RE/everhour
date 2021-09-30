use everhour::api::Request;
use everhour::config;
use everhour::status::status;

use std::process;

fn main() {
    let api = Request::new(config::API_KEY);
    let result = status(api);

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
