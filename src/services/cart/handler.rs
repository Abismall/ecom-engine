use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::PgConnection;

use crate::{
    error::{ConnectionPoolErrorWrapper, DatabaseErrorWrapper},
    ResourceIdentifierRequest,
};

use super::{
    model::Cart,
    query::{
        delete_cart_query, insert_cart_query, list_carts_with_orderlines_query, select_cart_query,
        set_cart_query,
    },
};

pub async fn create_cart(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match insert_cart_query(&mut conn) {
            Ok(cart) => HttpResponse::Ok().json(cart),
            Err(e) => DatabaseErrorWrapper(e).into(), // Use the error_response method from ResponseError
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn update_cart(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Json<Cart>,
) -> impl Responder {
    let params: Cart = payload.into_inner();
    match pool.get() {
        Ok(mut conn) => match select_cart_query(&mut conn, params.id) {
            Ok(existing_cart) => match set_cart_query(
                &mut conn,
                Cart {
                    id: existing_cart.id,
                    is_active: params.is_active,
                },
            ) {
                Ok(updated_cart) => HttpResponse::Ok().json(updated_cart),
                Err(e) => DatabaseErrorWrapper(e).into(),
            },
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn get_cart(
    path: web::Path<ResourceIdentifierRequest>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match select_cart_query(&mut conn, path.into_inner().id) {
            Ok(cart_option) => HttpResponse::Ok().json(cart_option),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn list_carts_with_orderlines(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match list_carts_with_orderlines_query(&mut conn).await {
            Ok(cart) => HttpResponse::Ok().json(cart),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn delete_cart(
    path: web::Path<ResourceIdentifierRequest>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match delete_cart_query(&mut conn, path.into_inner().id) {
            Ok(cart) => HttpResponse::Ok().json(cart),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}
