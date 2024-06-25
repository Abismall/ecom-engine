use std::env;

pub mod file;

pub fn fetch_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|e| {
        panic!("Failed to fetch environment variable: '{}'", e);
    })
}
