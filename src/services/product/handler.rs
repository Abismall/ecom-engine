use crate::{
    postgres::{execute_query, execute_query_with_args, ConnectionPool},
    ResourceIdentifierRequest,
};
use actix_web::{web, HttpResponse, Responder};

use super::{
    model::{NewProduct, Product},
    query::{
        delete_product_query, inner_join_product_and_stock_query, insert_product_query_pooled_conn,
        left_join_products_with_discounts, load_products_query,
        load_products_with_attributes_and_discounts_query, select_product_query, set_product_query,
    },
};

pub async fn delete_product(
    pool: web::Data<ConnectionPool>,
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
    pool: web::Data<ConnectionPool>,
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
    pool: web::Data<ConnectionPool>,
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
    pool: web::Data<ConnectionPool>,
    payload: web::Json<NewProduct>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, params| {
            insert_product_query_pooled_conn(
                NewProduct {
                    name: params.name,
                    in_stock: params.in_stock,

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

pub async fn list_products_with_discount(pool: web::Data<ConnectionPool>) -> impl Responder {
    match crate::postgres::execute_query(pool, left_join_products_with_discounts).await {
        Ok(products_with_discount) => products_with_discount,
        Err(e) => e.into(),
    }
}
pub async fn list_full_products(pool: web::Data<ConnectionPool>) -> impl Responder {
    match crate::postgres::execute_query(pool, load_products_with_attributes_and_discounts_query)
        .await
    {
        Ok(products_with_discount) => products_with_discount,
        Err(e) => e.into(),
    }
}
pub async fn list_products_with_stock(pool: web::Data<ConnectionPool>) -> impl Responder {
    match crate::postgres::execute_query(pool, inner_join_product_and_stock_query).await {
        Ok(products_with_stock) => products_with_stock,
        Err(e) => e.into(),
    }
}
