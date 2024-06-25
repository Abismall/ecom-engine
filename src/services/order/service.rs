use actix_web::web::{delete, get, post, put, scope};

use super::handler::create_orderline;
use super::handler::delete_orderline;
use super::handler::get_orderline;
use super::handler::update_orderline;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/orderline")
            .route("", post().to(create_orderline))
            .route("", put().to(update_orderline))
            .route("/{id}", get().to(get_orderline))
            .route("/{id}", delete().to(delete_orderline)),
    );
}
