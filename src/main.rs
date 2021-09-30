mod status;

use std::process;

use crate::status::status;

fn main() {
    let result = status();

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
