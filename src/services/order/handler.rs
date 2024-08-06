use actix_web::{web, Responder};
use diesel::PgConnection;

use crate::{
    error::DatabaseErrorWrapper,
    postgres::{execute_query_with_args, ConnectionPool},
    ResourceIdentifierRequest,
};

use super::{
    model::{NewOrderLine, OrderLine},
    query::{
        delete_orderline_query, insert_orderline_query, select_orderline_query, set_orderline_query,
    },
};

pub async fn create_orderline(
    pool: web::Data<ConnectionPool>,
    payload: web::Json<NewOrderLine>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| {
            insert_orderline_query(
                conn,
                NewOrderLine::new(
                    params.cart_id,
                    params.product_id,
                    Some(params.warehouse_id),
                    params.quantity,
                ),
            )
            .map_err(DatabaseErrorWrapper)
        },
        params,
    )
    .await
    {
        Ok(new_orderline) => new_orderline,
        Err(e) => e.into(),
    }
}
pub async fn get_orderline(
    pool: web::Data<ConnectionPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| select_orderline_query(conn, params.id).map_err(DatabaseErrorWrapper),
        params,
    )
    .await
    {
        Ok(new_orderline) => new_orderline,
        Err(e) => e.into(),
    }
}

pub async fn update_orderline(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Json<OrderLine>,
) -> impl Responder {
    let params: OrderLine = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| match select_orderline_query(conn, params.id).map_err(DatabaseErrorWrapper) {
            Ok(existing_orderline) => set_orderline_query(
                conn,
                OrderLine {
                    id: existing_orderline.id,
                    cart_id: existing_orderline.cart_id,
                    product_id: params.product_id,
                    warehouse_id: params.warehouse_id,
                    quantity: params.quantity,
                },
            )
            .map_err(DatabaseErrorWrapper),
            Err(e) => Err(e),
        },
        params,
    )
    .await
    {
        Ok(updated_orderline) => updated_orderline,
        Err(e) => e.into(),
    }
}

pub async fn delete_orderline(
    pool: web::Data<ConnectionPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| delete_orderline_query(conn, params.id).map_err(DatabaseErrorWrapper),
        params,
    )
    .await
    {
        Ok(deleted_count) => deleted_count,
        Err(e) => e.into(),
    }
}
