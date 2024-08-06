use actix_web::http::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE,
};
use actix_web::middleware::DefaultHeaders;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::postgres::ConnectionPool;

pub const DB_CON_RETRY_ATTEMPTS: u8 = 3;
pub const DEFAULT_PORT: u16 = 8080;

#[derive(Clone, Debug)]
pub struct RestApiHandler;

pub fn pg_r2d2_connection_pool(
    database_url: &str,
) -> Result<Pool<ConnectionManager<PgConnection>>, r2d2::Error> {
    let manager = create_connection_manager(database_url);
    r2d2::Pool::builder()
        .max_size(15) // Set an appropriate max size for your pool
        .build(manager)
}

fn create_connection_manager<T>(database_url: &str) -> ConnectionManager<T>
where
    T: diesel::Connection + diesel::r2d2::R2D2Connection + 'static,
    ConnectionManager<T>: diesel::r2d2::ManageConnection<Connection = T>,
{
    ConnectionManager::<T>::new(database_url)
}

pub fn pg_r2d2_connection_manager(url: &str) -> ConnectionManager<PgConnection> {
    create_connection_manager(url)
}

pub fn local_dev_cors() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .send_wildcard()
        .allow_any_origin()
        .allow_any_method()
        .allowed_headers(vec![
            CONTENT_TYPE,
            AUTHORIZATION,
            CACHE_CONTROL,
            ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN,
            CONTENT_LENGTH,
        ])
        .allowed_header("x-cache-status")
        .max_age(3600)
}

pub fn local_dev_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add((CONTENT_TYPE, HeaderValue::from_static("application/json")))
        .add((AUTHORIZATION, HeaderValue::from_static("Bearer")))
        .add((CACHE_CONTROL, HeaderValue::from_static("no-cache")))
        .add((ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*")))
}

pub async fn resolve_connection_pool(url: &str) -> ConnectionPool {
    let mut attempts = 0;
    let pool = loop {
        match crate::api::rest::pg_r2d2_connection_pool(url) {
            Ok(pool) => break pool,
            Err(err) => {
                attempts += 1;
                if attempts >= DB_CON_RETRY_ATTEMPTS {
                    log::error!("Failed to create connection pool: {}", err);
                    std::process::exit(1);
                } else {
                    log::warn!("Create connection attempt {} failed, retrying...", attempts);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    };
    pool
}
