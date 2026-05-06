pub mod config;
pub mod docker;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
