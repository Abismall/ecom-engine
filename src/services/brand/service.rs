use actix_web::web::{delete, get, post, put, scope};

use super::handler::{
    create_brand, delete_brand, get_brand, list_brand_products, list_brands, update_brand,
};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/brand")
            .route("", post().to(create_brand))
            .route("", get().to(list_brands))
            .route("", put().to(update_brand))
            .route("/{id}", get().to(get_brand))
            .route("/{id}", delete().to(delete_brand))
            .route("/{id}/products", get().to(list_brand_products)),
    );
}
