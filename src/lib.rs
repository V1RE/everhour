pub mod api;
pub mod config;
pub mod status;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
