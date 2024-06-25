use actix_web::http::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE,
};
use actix_web::middleware::DefaultHeaders;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

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
