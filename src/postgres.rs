use std::env;

use ::r2d2::Pool as r2d2Pool;
use actix_web::{web, HttpResponse, ResponseError};
use diesel::{
    r2d2::{self, ConnectionManager},
    Connection as r2d2Connection, PgConnection,
};
use dotenv::dotenv;
use r2d2::PooledConnection as r2d2PooledConnection;
use serde::Serialize;

use crate::error::ConnectionPoolErrorWrapper;
pub type ConnectionPool = r2d2Pool<ConnectionManager<PgConnection>>;
pub type PooledConnection = r2d2PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;

pub fn create_connection_pool<T>(manager: ConnectionManager<T>) -> r2d2Pool<ConnectionManager<T>>
where
    T: diesel::Connection + diesel::r2d2::R2D2Connection + 'static,
    ConnectionManager<T>: diesel::r2d2::ManageConnection<Connection = T>,
{
    r2d2::Pool::builder()
        .build(manager)
        .expect("Error: connection pool build failed")
}

pub fn create_connection_manager<T>(database_url: &str) -> r2d2Pool<ConnectionManager<T>>
where
    T: diesel::Connection + diesel::r2d2::R2D2Connection + 'static,
    ConnectionManager<T>: diesel::r2d2::ManageConnection<Connection = T>,
{
    let manager = ConnectionManager::<T>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Error: connection pool build failed")
}

pub fn new_connection() -> Result<PgConnection, diesel::ConnectionError> {
    dotenv().ok();
    PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
}

pub async fn execute_query<F, T, E>(
    pool: web::Data<ConnectionPool>,
    f: F,
) -> Result<HttpResponse, E>
where
    F: FnOnce(&mut PooledConnection) -> Result<T, E>,
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
    pool: web::Data<ConnectionPool>,
    f: F,
    args: A,
) -> Result<HttpResponse, E>
where
    F: FnOnce(&mut PooledConnection, A) -> Result<T, E>,
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
