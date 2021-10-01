use everhour::status::status;

use std::process;

fn main() {
    let result = status(false);

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
