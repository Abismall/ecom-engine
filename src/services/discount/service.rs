use actix_web::web::{delete, get, post, put, scope};

use crate::services::brand::handler::create_discount_brand;

use super::handler::create_discount;
use super::handler::create_discount_product;
use super::handler::delete_discount;
use super::handler::get_discount;
use super::handler::list_discounts;
use super::handler::update_discount;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/discount")
            .route("", post().to(create_discount))
            .route("", get().to(list_discounts))
            .route("", put().to(update_discount))
            .route("/{id}", get().to(get_discount))
            .route("/{id}", delete().to(delete_discount))
            .route("/product", post().to(create_discount_product))
            .route("/brand", post().to(create_discount_brand)),
    );
}
