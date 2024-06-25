use crate::{
    data::db::{execute_query, execute_query_with_args, PgPool},
    error::ConnectionPoolErrorWrapper,
    ResourceIdentifierRequest,
};
use actix_web::{web, HttpResponse, Responder, ResponseError};

use super::{
    model::{NewProduct, Product},
    query::{
        delete_product_query, inner_join_product_and_stock_query, insert_product_query,
        left_join_products_with_discounts, load_products_query, select_product_query,
        set_product_query,
    },
};

pub async fn delete_product(
    pool: web::Data<PgPool>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match execute_query_with_args(pool, |conn, id| delete_product_query(conn, id), params.id).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}

pub async fn get_product(
    pool: web::Data<PgPool>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match execute_query_with_args(pool, |conn, id| select_product_query(id, conn), params.id).await
    {
        Ok(product) => product,
        Err(e) => e.into(),
    }
}

pub async fn update_product(
    pool: web::Data<PgPool>,
    payload: web::Json<Product>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| {
            set_product_query(
                Product {
                    name: params.name,
                    in_stock: params.in_stock,
                    size: params.size,
                    color: params.color,
                    weight: params.weight,
                    weight_unit: params.weight_unit,
                    width: params.width,
                    height: params.height,
                    category_id: params.category_id,
                    brand_id: params.brand_id,
                    price: params.price,
                    tax_rate: params.tax_rate,
                    id: params.id,
                },
                conn,
            )
        },
        params,
    )
    .await
    {
        Ok(updated_product) => updated_product,
        Err(e) => e.into(),
    }
}
pub async fn create_product(
    pool: web::Data<PgPool>,
    payload: web::Json<NewProduct>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| {
            insert_product_query(
                NewProduct {
                    name: params.name,
                    in_stock: params.in_stock,
                    size: params.size,
                    color: params.color,
                    weight: params.weight,
                    weight_unit: params.weight_unit,
                    width: params.width,
                    height: params.height,
                    category_id: params.category_id,
                    brand_id: params.brand_id,
                    price: params.price,
                    tax_rate: params.tax_rate,
                },
                conn,
            )
        },
        params,
    )
    .await
    {
        Ok(updated_product) => updated_product,
        Err(e) => e.into(),
    }
}

pub async fn list_products(
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    match execute_query(pool, load_products_query).await {
        Ok(products) => products,
        Err(e) => e.into(),
    }
}

pub async fn list_products_with_discount(pool: web::Data<PgPool>) -> impl Responder {
    match crate::data::db::execute_query(pool, left_join_products_with_discounts).await {
        Ok(products_with_discount) => products_with_discount,
        Err(e) => e.into(),
    }
}

pub async fn list_products_with_stock(pool: web::Data<PgPool>) -> impl Responder {
    match crate::data::db::execute_query(pool, inner_join_product_and_stock_query).await {
        Ok(products_with_stock) => products_with_stock,
        Err(e) => e.into(),
    }
}
