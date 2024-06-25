use super::handler::create_product;
use super::handler::delete_product;
use super::handler::get_product;
use super::handler::list_products;
use super::handler::list_products_with_discount;
use super::handler::list_products_with_stock;
use super::handler::update_product;

use actix_web::web::{delete, get, post, put, scope};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/product")
            .route("", post().to(create_product))
            .route("", get().to(list_products_with_discount))
            .route("", put().to(update_product))
            .route("/{id}", get().to(get_product))
            .route("/{id}", delete().to(delete_product))
            .route("/with_stock", get().to(list_products_with_stock))
            .route("/raw", get().to(list_products)),
    );
}
