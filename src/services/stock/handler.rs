use crate::{
    postgres::{execute_query, execute_query_with_args, ConnectionPool},
    services::order::model::NewWarehouse,
    ResourceIdentifierRequest,
};
use actix_web::{web, HttpResponse, Responder};

use super::{
    model::{NewStockQuantity, StockQuantity},
    query::{
        delete_stock_quantity_from_product_query, delete_warehouse_query,
        insert_stock_quantity_query, insert_warehouse_query, load_warehouses_query,
        select_stock_quantity_for_product, select_stock_quantity_for_warehouse_query,
        select_warehouse_query, set_stock_quantity_for_product,
    },
};

pub async fn create_stock_quantity(
    pool: web::Data<ConnectionPool>,
    payload: web::Json<NewStockQuantity>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, new_quantity| insert_stock_quantity_query(conn, new_quantity),
        params,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}

pub async fn delete_stock_quantity_from_product(
    pool: web::Data<ConnectionPool>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match execute_query_with_args(
        pool,
        |conn, id| delete_stock_quantity_from_product_query(conn, id),
        params.id,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}
pub async fn list_stock_quantity(pool: web::Data<ConnectionPool>) -> impl Responder {
    match execute_query(pool, load_warehouses_query).await {
        Ok(stock_quantities) => stock_quantities,
        Err(e) => e.into(),
    }
}

pub async fn get_stock_quantity_for_product(
    pool: web::Data<ConnectionPool>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match execute_query_with_args(
        pool,
        |conn, id| select_stock_quantity_for_product(id, conn),
        params.id,
    )
    .await
    {
        Ok(product) => product,
        Err(e) => e.into(),
    }
}

pub async fn update_stock_quantity_for_product(
    pool: web::Data<ConnectionPool>,
    payload: web::Json<StockQuantity>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, new_quantity| set_stock_quantity_for_product(conn, new_quantity),
        params,
    )
    .await
    {
        Ok(updated_product) => updated_product,
        Err(e) => e.into(),
    }
}

pub async fn create_warehouse(
    pool: web::Data<ConnectionPool>,
    payload: web::Json<NewWarehouse>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, new_warehouse| insert_warehouse_query(conn, new_warehouse),
        params,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}

pub async fn get_stock_quantity_for_warehouse(
    pool: web::Data<ConnectionPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, path| select_stock_quantity_for_warehouse_query(conn, path.id),
        params,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}
pub async fn get_warehouse(
    pool: web::Data<ConnectionPool>,
    payload: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = payload.into_inner();
    match execute_query_with_args(
        pool,
        |conn, path| select_warehouse_query(conn, path.id),
        params,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}

pub async fn list_warehouses(pool: web::Data<ConnectionPool>) -> impl Responder {
    match execute_query(pool, load_warehouses_query).await {
        Ok(products) => products,
        Err(e) => e.into(),
    }
}
pub async fn delete_warehouse(
    pool: web::Data<ConnectionPool>,
    path: web::Path<ResourceIdentifierRequest>,
) -> impl Responder {
    let params = path.into_inner();
    match execute_query_with_args(pool, |conn, id| delete_warehouse_query(conn, id), params.id)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.into(),
    }
}
