pub mod api;
pub mod cfg;
pub mod data;
pub mod docker;
pub mod error;
pub mod http;
pub mod services;
pub mod logger;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceIdentifierRequest {
    pub id: i32,
}

pub const CONFIG_FILE_PATH: &str = "config.json";

