use actix_web::{web, HttpResponse, Responder, ResponseError};
use diesel::PgConnection;

use crate::{
    services::discount::query::{insert_discount_product_query, insert_discount_query},
    error::{ConnectionPoolErrorWrapper, DatabaseErrorWrapper},
    ResourceIdentifierRequest,
};

use super::{
    model::{relations::DiscountProduct, Discount, NewDiscount},
    query::{
        delete_discount_query, load_discounts_query, select_discount_query, set_discount_query,
    },
};

pub async fn get_discount(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match select_discount_query(&mut conn, payload.into_inner().id) {
            Ok(discount) => HttpResponse::Ok().json(discount),
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn list_discounts(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match load_discounts_query(&mut conn) {
            Ok(discounts) => HttpResponse::Ok().json(discounts),
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn delete_discount(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match delete_discount_query(&mut conn, payload.into_inner().id) {
            Ok(deleted_count) => {
                if deleted_count > 0 {
                    HttpResponse::Ok().finish()
                } else {
                    HttpResponse::NotFound().body("Discount not found")
                }
            }
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn create_discount_product(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    discount_product: web::Json<DiscountProduct>,
) -> impl Responder {
    println!("Received JSON: {:?}", discount_product);
    let new_discount_product = discount_product.into_inner();
    match pool.get() {
        Ok(mut conn) => match insert_discount_product_query(&mut conn, new_discount_product) {
            Ok(discount_product) => HttpResponse::Ok().json(discount_product),
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}
pub async fn create_discount(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    discount_product: web::Json<NewDiscount>,
) -> impl Responder {
    println!("Received JSON: {:?}", discount_product);
    let new_discount_product = discount_product.into_inner();
    match pool.get() {
        Ok(mut conn) => match insert_discount_query(&mut conn, new_discount_product) {
            Ok(discount_product) => HttpResponse::Ok().json(discount_product),
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}

pub async fn update_discount(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Json<Discount>,
) -> impl Responder {
    let params: Discount = payload.into_inner();
    match pool.get() {
        Ok(mut conn) => match select_discount_query(&mut conn, params.id) {
            Ok(existing_discount) => match set_discount_query(
                &mut conn,
                Discount {
                    id: existing_discount.id,
                    name: params.name,
                    discount_type: params.discount_type,
                    value: params.value,
                    start_date: params.start_date,
                    end_date: params.end_date,
                    min_quantity: params.min_quantity,
                },
            ) {
                Ok(updated_discount) => HttpResponse::Ok().json(updated_discount),
                Err(e) => DatabaseErrorWrapper(e).error_response(),
            },
            Err(e) => DatabaseErrorWrapper(e).error_response(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).error_response(),
    }
}
