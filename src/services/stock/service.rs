use actix_web::web::{delete, get, post, put, scope};

use super::handler::{
    create_stock_quantity, create_warehouse, delete_stock_quantity_from_product, delete_warehouse,
    get_stock_quantity_for_product, get_warehouse, list_stock_quantity, list_warehouses,
    update_stock_quantity_for_product,
};

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/stock")
            .route("", post().to(create_stock_quantity))
            .route("", get().to(list_stock_quantity))
            .route("", put().to(update_stock_quantity_for_product))
            .route("/{id}", get().to(get_stock_quantity_for_product))
            .route("/{id}", delete().to(delete_stock_quantity_from_product))
            .route("/warehouse", get().to(list_warehouses))
            .route("/warehouse", post().to(create_warehouse))
            .route("/warehouse/{id}", get().to(get_warehouse))
            .route("/warehouse/{id}", delete().to(delete_warehouse)),
    );
}
