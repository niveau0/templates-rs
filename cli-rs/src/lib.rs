use crate::constants::AUTHORS;
use crate::constants::VERSION;

pub mod cfg;
pub mod command;
pub mod constants;
pub mod ctx_example;
pub mod error;

pub fn version() -> &'static str {
    VERSION
}

pub fn authors() -> &'static str {
    AUTHORS
}
