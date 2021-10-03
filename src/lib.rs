pub extern crate lazy_static;

pub mod api;
pub mod config;
pub mod status;
pub mod timer;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
