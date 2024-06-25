use actix_web::web::{delete, get, post, put};

use super::handler::list_carts_with_orderlines;
use super::handler::{create_cart, delete_cart, get_cart, update_cart};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/cart")
            .route("", post().to(create_cart))
            .route("", get().to(list_carts_with_orderlines))
            .route("", put().to(update_cart))
            .route("/{id}", get().to(get_cart))
            .route("/{id}", delete().to(delete_cart)),
    );
}
