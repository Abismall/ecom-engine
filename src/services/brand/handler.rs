use actix_web::{web, HttpResponse, Responder};
use diesel::PgConnection;

use crate::{
    error::{ConnectionPoolErrorWrapper, DatabaseErrorWrapper},
    services::{
        brand::query::insert_discount_brand_query, discount::model::relations::DiscountBrand,
    },
    ResourceIdentifierRequest,
};

use super::{
    model::{Brand, NewBrand},
    query::{
        delete_brand_query, insert_brand_query, load_brand_products_query, load_brand_query,
        select_brand_query, set_brand_query,
    },
};

pub async fn list_brand_products(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match pool.get() {
        Ok(mut conn) => match load_brand_products_query(&mut conn, params.id) {
            Ok(products) => HttpResponse::Ok().json(products),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}

pub async fn delete_brand(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match pool.get() {
        Ok(mut conn) => match delete_brand_query(&mut conn, params.id) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}

pub async fn get_brand(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match pool.get() {
        Ok(mut conn) => match select_brand_query(&mut conn, params.id).await {
            Ok(brand) => HttpResponse::Ok().json(brand),

            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}

pub async fn update_brand(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Json<Brand>,
) -> impl Responder {
    let params = payload.into_inner();
    match pool.get() {
        Ok(mut conn) => match select_brand_query(&mut conn, params.id).await {
            Ok(brand) => match set_brand_query(&mut conn, brand) {
                Ok(updated_brand) => HttpResponse::Ok().json(updated_brand),
                Err(e) => DatabaseErrorWrapper(e).into(),
            },
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}
pub async fn create_brand(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    payload: web::Json<NewBrand>,
) -> impl Responder {
    let params = payload.into_inner();
    match pool.get() {
        Ok(mut conn) => match insert_brand_query(
            &mut conn,
            NewBrand {
                name: params.name,
                description: params.description,
            },
        ) {
            Ok(brand) => HttpResponse::Ok().json(brand),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}

pub async fn list_brands(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> impl Responder {
    match pool.get() {
        Ok(mut conn) => match load_brand_query(&mut conn) {
            Ok(brands) => HttpResponse::Ok().json(brands),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}
pub async fn create_discount_brand(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
    discount_brand: web::Json<DiscountBrand>,
) -> impl Responder {
    println!("Received JSON: {:?}", discount_brand);
    let new_discount_brand = discount_brand.into_inner();
    match pool.get() {
        Ok(mut conn) => match insert_discount_brand_query(&mut conn, new_discount_brand) {
            Ok(discount_brand) => HttpResponse::Ok().json(discount_brand),
            Err(e) => DatabaseErrorWrapper(e).into(),
        },
        Err(e) => ConnectionPoolErrorWrapper(e).into(),
    }
}
