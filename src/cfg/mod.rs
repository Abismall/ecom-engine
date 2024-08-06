use std::env;


pub fn fetch_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|e| {
        panic!("Failed to fetch environment variable: '{}'", e);
    })
}
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub environment: String,
    pub rest_api: RestApiConfig,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RestApiConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RedisConfig {
    pub redis_url: String,
    pub redis_port: String,
    pub redis_host: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PostgresConfig {
    pub db_url: String,
    pub db_secret: String,
    pub db_password: String,
    pub db_user: String,
    pub db_name: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let config_content = std::fs::read_to_string(file_path).unwrap_or_else(|err| {
            panic!(
                "Failed to read the configuration file '{}': {}",
                file_path, err
            );
        });

        serde_json::from_str(&config_content).unwrap_or_else(|err| {
            panic!(
                "Failed to parse the configuration file '{}': {}",
                file_path, err
            );
        })
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Env {
    pub api_host: String,
    pub api_port: String,
    pub redis_url: String,
    pub redis_port: String,
    pub redis_host: String,
    pub db_url: String,
    pub db_secret: String,
    pub db_password: String,
    pub db_user: String,
    pub db_name: String,
}

impl Env {
    pub fn from_config(config: &Config) -> Self {
        dotenv().ok(); // Load environment variables from .env file
        let api_host = Self::fetch_env_var(&config.rest_api.host);
        let api_port = Self::fetch_env_var(&config.rest_api.port);
        let redis_url = Self::fetch_env_var(&config.redis.redis_url);
        let redis_port = Self::fetch_env_var(&config.redis.redis_port);
        let redis_host = Self::fetch_env_var(&config.redis.redis_host);
        let db_url = Self::fetch_env_var(&config.postgres.db_url);
        let db_secret = Self::fetch_env_var(&config.postgres.db_secret);
        let db_password = Self::fetch_env_var(&config.postgres.db_password);
        let db_user = Self::fetch_env_var(&config.postgres.db_user);
        let db_name = Self::fetch_env_var(&config.postgres.db_name);

        Env {
            api_host,
            api_port,
            redis_url,
            redis_port,
            redis_host,
            db_url,
            db_secret,
            db_password,
            db_user,
            db_name,
        }
    }

    fn fetch_env_var(key: &str) -> String {
        env::var(key).unwrap_or_else(|e| {
            panic!("Failed to fetch environment variable '{}': {}", key, e);
        })
    }
}

impl From<Config> for Env {
    fn from(config: Config) -> Self {
        Env::from_config(&config)
    }
}
