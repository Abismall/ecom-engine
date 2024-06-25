use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::PgConnection;

use crate::{
    data::db::{execute_query_with_args, PgPool},
    error::ConnectionPoolErrorWrapper,
    ResourceIdentifierRequest,
};

use super::{
    model::{NewOrderLine, OrderLine},
    query::{
        delete_orderline_query, insert_orderline_query, select_orderline_query, set_orderline_query,
    },
};

pub async fn create_orderline(
    pool: web::Data<PgPool>,
    payload: web::Json<NewOrderLine>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| insert_orderline_query(conn, params),
        params,
    )
    .await
    {
        Ok(new_orderline) => new_orderline,
        Err(e) => e.into(),
    }
}
pub async fn get_orderline(
    pool: web::Data<PgPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| select_orderline_query(conn, params.id),
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
        |conn, params| match select_orderline_query(conn, params.id) {
            Ok(existing_orderline) => set_orderline_query(
                conn,
                OrderLine {
                    id: existing_orderline.id,
                    cart_id: existing_orderline.cart_id,
                    product_id: params.product_id,
                    quantity: params.quantity,
                },
            ),
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
    pool: web::Data<PgPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| delete_orderline_query(conn, params.id),
        params,
    )
    .await
    {
        Ok(deleted_count) => deleted_count,
        Err(e) => e.into(),
    }
}
