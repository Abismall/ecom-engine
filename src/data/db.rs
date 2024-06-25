use std::env;

use ::r2d2::Pool;
use actix_web::{web, HttpResponse, ResponseError};
use diesel::{
    r2d2::{self, ConnectionManager},
    Connection, PgConnection,
};
use dotenv::dotenv;
use r2d2::PooledConnection;
use serde::Serialize;

use crate::error::ConnectionPoolErrorWrapper;

pub fn create_connection_pool<T>(manager: ConnectionManager<T>) -> Pool<ConnectionManager<T>>
where
    T: diesel::Connection + diesel::r2d2::R2D2Connection + 'static,
    ConnectionManager<T>: diesel::r2d2::ManageConnection<Connection = T>,
{
    r2d2::Pool::builder()
        .build(manager)
        .expect("Error: connection pool build failed")
}

pub fn create_connection_manager<T>(database_url: &str) -> Pool<ConnectionManager<T>>
where
    T: diesel::Connection + diesel::r2d2::R2D2Connection + 'static,
    ConnectionManager<T>: diesel::r2d2::ManageConnection<Connection = T>,
{
    let manager = ConnectionManager::<T>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Error: connection pool build failed")
}

pub fn new_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn execute_query<F, T, E>(
    pool: web::Data<PgPool>,
    f: F,
) -> Result<HttpResponse, E>
where
    F: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<T, E>,
    T: Serialize,
    E: ResponseError + From<ConnectionPoolErrorWrapper>,
{
    match pool.get() {
        Ok(mut conn) => match f(&mut conn) {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(err) => Err(err),
        },
        Err(e) => Err(ConnectionPoolErrorWrapper(e).into()),
    }
}

// Function for queries with additional arguments
pub async fn execute_query_with_args<F, T, E, A>(
    pool: web::Data<PgPool>,
    f: F,
    args: A,
) -> Result<HttpResponse, E>
where
    F: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>, A) -> Result<T, E>,
    T: Serialize,
    E: ResponseError + From<ConnectionPoolErrorWrapper>,
{
    match pool.get() {
        Ok(mut conn) => match f(&mut conn, args) {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(err) => Err(err),
        },
        Err(e) => Err(ConnectionPoolErrorWrapper(e).into()),
    }
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
