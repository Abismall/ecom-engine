use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use std::fmt;
use thiserror::Error;

pub mod message {
    // Http request error messages
    pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
    pub const SERVICE_UNAVAILABLE: &str = "Service Unavailable";
    pub const UNPROCESSABLE_ENTITY: &str = "Unprocessable Entity";
    pub const FORBIDDEN: &str = "Forbidden";
    pub const UNAUTHORIZED: &str = "Unauthorized";
    pub const BAD_REQUEST: &str = "Bad Request";
    pub const TIME_OUT: &str = "Connection Timed Out";
    pub const NOT_FOUND: &str = "Not Found";

    // Database error messages
    pub const UNIQUE_VIOLATION: &str = "Unique Violation";
    pub const FOREIGN_KEY_VIOLATION: &str = "Foreign Key Violation";
    pub const DATABASE_ERROR: &str = "Database Error";
    pub const SERIALIZATION_ERROR: &str = "Serialization Error";
    pub const QUERY_BUILDER_ERROR: &str = "Query Builder Error";

    pub const CONNECTION_POOL_ERROR: &str = "Connection Pool Error";
    pub const CONNECTION_ERROR: &str = "Connection Error";

    // Redis error messages
    pub const REDIS_IO_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_CLIENT_ERROR: &str = BAD_REQUEST;
    pub const REDIS_AUTHENTICATION_FAILED: &str = UNAUTHORIZED;
    pub const REDIS_TYPE_ERROR: &str = BAD_REQUEST;
    pub const REDIS_EXEC_ABORT_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_BUSY_LOADING_ERROR: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_NO_SCRIPT_ERROR: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_MOVED_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_ASK_ERROR: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_TRY_AGAIN: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_CLUSTER_DOWN: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_CROSS_SLOT_ERROR: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_MASTER_DOWN: &str = SERVICE_UNAVAILABLE;
    pub const REDIS_READ_ONLY_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_EXTENSION_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_RESPONSE_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_PARSE_ERROR: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_INVALID_CLIENT_CONFIG: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_MASTER_NAME_NOT_FOUND: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_NO_VALID_REPLICAS: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_EMPTY_SENTINEL_LIST: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_NOT_BUSY: &str = INTERNAL_SERVER_ERROR;
    pub const REDIS_CLUSTER_CONNECTION_NOT_FOUND: &str = INTERNAL_SERVER_ERROR;
}

#[derive(Debug, Error)]
pub struct EnvVarError {
    pub var_name: String,
    pub source: std::env::VarError,
}

impl fmt::Display for EnvVarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to read value for {}: {}",
            self.var_name, self.source
        )
    }
}

impl ResponseError for EnvVarError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(format!("EnvVar Error: {}", self))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl Into<HttpResponse> for EnvVarError {
    fn into(self) -> HttpResponse {
        self.error_response()
    }
}

#[derive(Debug, Error)]
#[error("Database Error: {0}")]
pub struct DatabaseErrorWrapper(pub DieselError);

impl ResponseError for DatabaseErrorWrapper {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            DieselError::NotFound => HttpResponse::NotFound().body(message::NOT_FOUND),
            DieselError::DatabaseError(kind, _info) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    HttpResponse::Conflict().body(message::UNIQUE_VIOLATION)
                }
                diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                    HttpResponse::BadRequest().body(message::FOREIGN_KEY_VIOLATION)
                }
                _ => HttpResponse::InternalServerError().body(message::INTERNAL_SERVER_ERROR),
            },
            DieselError::SerializationError(err) => HttpResponse::InternalServerError()
                .body(format!("{}: {}", message::SERIALIZATION_ERROR, err)),
            DieselError::QueryBuilderError(err) => HttpResponse::BadRequest().body(format!(
                "{}: {}",
                message::QUERY_BUILDER_ERROR,
                err
            )),
            _ => HttpResponse::InternalServerError().body(message::INTERNAL_SERVER_ERROR),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self.0 {
            DieselError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            DieselError::DatabaseError(_, _) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            DieselError::SerializationError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            DieselError::QueryBuilderError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl Into<HttpResponse> for DatabaseErrorWrapper {
    fn into(self) -> HttpResponse {
        self.error_response()
    }
}

#[derive(Debug, Error)]
#[error("Connection Pool Error: {0}")]
pub struct ConnectionPoolErrorWrapper(pub R2d2Error);

impl ResponseError for ConnectionPoolErrorWrapper {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(message::CONNECTION_POOL_ERROR)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug, Error)]
#[error("Redis Error: {0}")]
pub struct RedisErrorWrapper(pub RedisError);

impl ResponseError for RedisErrorWrapper {
    fn error_response(&self) -> HttpResponse {
        match self.0.kind() {
            redis::ErrorKind::IoError => {
                HttpResponse::InternalServerError().body(message::REDIS_IO_ERROR)
            }
            redis::ErrorKind::ClientError => {
                HttpResponse::InternalServerError().body(message::REDIS_CLIENT_ERROR)
            }
            redis::ErrorKind::AuthenticationFailed => {
                HttpResponse::Unauthorized().body(message::REDIS_AUTHENTICATION_FAILED)
            }
            redis::ErrorKind::TypeError => {
                HttpResponse::BadRequest().body(message::REDIS_TYPE_ERROR)
            }
            redis::ErrorKind::ExecAbortError => {
                HttpResponse::InternalServerError().body(message::REDIS_EXEC_ABORT_ERROR)
            }
            redis::ErrorKind::BusyLoadingError => {
                HttpResponse::ServiceUnavailable().body(message::REDIS_BUSY_LOADING_ERROR)
            }
            redis::ErrorKind::NoScriptError => {
                HttpResponse::InternalServerError().body(message::REDIS_NO_SCRIPT_ERROR)
            }
            redis::ErrorKind::Moved => {
                HttpResponse::InternalServerError().body(message::REDIS_MOVED_ERROR)
            }
            redis::ErrorKind::Ask => {
                HttpResponse::InternalServerError().body(message::REDIS_ASK_ERROR)
            }
            redis::ErrorKind::TryAgain => {
                HttpResponse::ServiceUnavailable().body(message::REDIS_TRY_AGAIN)
            }
            redis::ErrorKind::ClusterDown => {
                HttpResponse::ServiceUnavailable().body(message::REDIS_CLUSTER_DOWN)
            }
            redis::ErrorKind::CrossSlot => {
                HttpResponse::BadRequest().body(message::REDIS_CROSS_SLOT_ERROR)
            }
            redis::ErrorKind::MasterDown => {
                HttpResponse::ServiceUnavailable().body(message::REDIS_MASTER_DOWN)
            }
            redis::ErrorKind::ReadOnly => {
                HttpResponse::ServiceUnavailable().body(message::REDIS_READ_ONLY_ERROR)
            }
            redis::ErrorKind::ExtensionError => {
                HttpResponse::InternalServerError().body(message::REDIS_EXTENSION_ERROR)
            }
            redis::ErrorKind::ResponseError => {
                HttpResponse::InternalServerError().body(message::REDIS_RESPONSE_ERROR)
            }
            redis::ErrorKind::ParseError => {
                HttpResponse::InternalServerError().body(message::REDIS_PARSE_ERROR)
            }
            redis::ErrorKind::InvalidClientConfig => {
                HttpResponse::InternalServerError().body(message::REDIS_INVALID_CLIENT_CONFIG)
            }
            redis::ErrorKind::MasterNameNotFoundBySentinel => {
                HttpResponse::InternalServerError().body(message::REDIS_MASTER_NAME_NOT_FOUND)
            }
            redis::ErrorKind::NoValidReplicasFoundBySentinel => {
                HttpResponse::InternalServerError().body(message::REDIS_NO_VALID_REPLICAS)
            }
            redis::ErrorKind::EmptySentinelList => {
                HttpResponse::InternalServerError().body(message::REDIS_EMPTY_SENTINEL_LIST)
            }
            redis::ErrorKind::NotBusy => {
                HttpResponse::InternalServerError().body(message::REDIS_NOT_BUSY)
            }
            redis::ErrorKind::ClusterConnectionNotFound => HttpResponse::InternalServerError()
                .body(message::REDIS_CLUSTER_CONNECTION_NOT_FOUND),
            _ => HttpResponse::InternalServerError().body(message::INTERNAL_SERVER_ERROR),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.0.kind() {
            redis::ErrorKind::IoError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::ClientError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::AuthenticationFailed => actix_web::http::StatusCode::UNAUTHORIZED,
            redis::ErrorKind::TypeError => actix_web::http::StatusCode::BAD_REQUEST,
            redis::ErrorKind::ExecAbortError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::BusyLoadingError => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            redis::ErrorKind::NoScriptError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::Moved => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::Ask => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::TryAgain => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            redis::ErrorKind::ClusterDown => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            redis::ErrorKind::CrossSlot => actix_web::http::StatusCode::BAD_REQUEST,
            redis::ErrorKind::MasterDown => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            redis::ErrorKind::ReadOnly => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            redis::ErrorKind::ExtensionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::ResponseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::ParseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::InvalidClientConfig => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            redis::ErrorKind::MasterNameNotFoundBySentinel => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            redis::ErrorKind::NoValidReplicasFoundBySentinel => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            redis::ErrorKind::EmptySentinelList => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            redis::ErrorKind::NotBusy => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            redis::ErrorKind::ClusterConnectionNotFound => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ConnectionPoolErrorWrapper> for HttpResponse {
    fn from(error: ConnectionPoolErrorWrapper) -> Self {
        error.error_response()
    }
}
impl From<ConnectionPoolErrorWrapper> for DatabaseErrorWrapper {
    fn from(error: ConnectionPoolErrorWrapper) -> Self {
        DatabaseErrorWrapper(DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(error.0.to_string()),
        ))
    }
}
