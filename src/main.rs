mod api;
mod status;

use std::process;

use crate::api::Api;
use crate::status::status;

fn main() {
    let api = Api::new();
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
