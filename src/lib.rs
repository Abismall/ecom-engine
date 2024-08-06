pub mod api;
pub mod cfg;
pub mod docker;
pub mod error;
pub mod http;
pub mod logger;
pub mod postgres;
pub mod redis;
pub mod schema;
pub mod services;
pub mod stream;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceIdentifierRequest {
    pub id: i32,
}

pub const CONFIG_FILE_PATH: &str = "config.json";

pub trait Localhost {
    fn is_localhost(&self) -> bool;
    fn as_str(&self) -> &str;
}

impl Localhost for String {
    fn is_localhost(&self) -> bool {
        matches!(self.as_str(), "localhost" | "127.0.0.1" | "0.0.0.0")
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl<'a> Localhost for &'a str {
    fn is_localhost(&self) -> bool {
        matches!(*self, "localhost" | "127.0.0.1" | "0.0.0.0")
    }

    fn as_str(&self) -> &str {
        self
    }
}

// Implementing Display for the Localhost trait
impl std::fmt::Display for dyn Localhost + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
