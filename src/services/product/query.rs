use std::collections::HashMap;

use crate::error::DatabaseErrorWrapper;
use crate::services::discount::model::Discount;
use crate::services::stock::model::StockQuantity;
use diesel::prelude::*;
use diesel::PgConnection;
use r2d2::PooledConnection;

use super::model::NewProduct;
use super::model::Product;
use super::model::ProductWithDiscount;
// pub fn insert_product_query_internal<C>(
//     new_product: NewProduct,
//     connection: &mut C,
// ) -> Result<Product, DatabaseErrorWrapper>
// where
//     C: diesel::Connection<Backend = diesel::pg::Pg, TransactionManager = <PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> as Connection>::TransactionManager> + diesel::connection::LoadConnection,
// {
//     diesel::insert_into(crate::data::schema::products::table)
//         .values(&new_product)
//         .returning(crate::data::schema::products::all_columns)
//         .get_result::<Product>(connection)
//         .map_err(DatabaseErrorWrapper)
// }
pub fn insert_product_query_internal<C>(
    new_product: NewProduct,
    connection: &mut C,
) -> Result<Product, DatabaseErrorWrapper>
where
    C: Connection<Backend = diesel::pg::Pg> + diesel::connection::LoadConnection,
{
    diesel::insert_into(crate::data::schema::products::table)
        .values(&new_product)
        .returning(crate::data::schema::products::all_columns)
        .get_result::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}
pub fn insert_product_query(
    new_product: NewProduct,
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Product, DatabaseErrorWrapper> {
    insert_product_query_internal(new_product, connection)
}

pub fn insert_product_query_pg(
    new_product: NewProduct,
    connection: &mut PgConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    insert_product_query_internal(new_product, connection)
}
pub fn select_product_query(
    product_id: i32,
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Product, DatabaseErrorWrapper> {
    crate::data::schema::products::table
        .select(Product::as_select())
        .filter(crate::data::schema::products::id.eq(product_id))
        .first::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn inner_join_product_and_stock_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<(Product, StockQuantity)>, DatabaseErrorWrapper> {
    crate::data::schema::products::table
        .inner_join(crate::data::schema::stock_quantities::table)
        .select((Product::as_select(), StockQuantity::as_select()))
        .load::<(Product, StockQuantity)>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn set_product_query(
    updated_product: Product,
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Product, DatabaseErrorWrapper> {
    set_product_query_internal(updated_product, connection)
}
pub fn set_product_query_pg(
    updated_product: Product,
    connection: &mut PgConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    set_product_query_internal(updated_product, connection)
}
pub fn set_product_query_internal<C>(
    updated_product: Product,
    connection: &mut C,
) -> Result<Product, DatabaseErrorWrapper>
where
    C: Connection<Backend = diesel::pg::Pg> + diesel::connection::LoadConnection,
{
    diesel::update(crate::data::schema::products::table.find(updated_product.id))
        .set(&updated_product)
        .get_result::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn delete_product_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    product_id: i32,
) -> Result<usize, DatabaseErrorWrapper> {
    diesel::delete(crate::data::schema::products::table.find(product_id))
        .execute(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn load_products_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<Product>, DatabaseErrorWrapper> {
    crate::data::schema::products::table
        .load::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn left_join_products_with_discounts(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<ProductWithDiscount>, DatabaseErrorWrapper> {
    println!("Function is executing, result is not from cache"); // Log when the function is executed

    use crate::data::schema::discount_products::dsl::{
        discount_id as dp_discount_id, discount_products, product_id as dp_product_id,
    };
    use crate::data::schema::discounts::dsl::{discounts, id as discount_id};
    use crate::data::schema::products::dsl::*;

    let results = products
        .left_join(discount_products.on(id.eq(dp_product_id)))
        .left_join(discounts.on(dp_discount_id.eq(discount_id)))
        .select((
            crate::data::schema::products::all_columns,
            crate::data::schema::discounts::all_columns.nullable(),
        ))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    let mut product_map: HashMap<i32, ProductWithDiscount> = HashMap::new();

    for (product, discount) in results {
        let product_id = product.id;
        product_map
            .entry(product_id)
            .and_modify(|entry| {
                if let Some(discount) = &discount {
                    entry.discounts.push(discount.clone());
                }
            })
            .or_insert_with(|| ProductWithDiscount {
                product,
                discounts: discount.into_iter().collect(),
            });
    }

    Ok(product_map.into_iter().map(|(_, v)| v).collect())
}

pub fn fetch_product_discounts_query(
    product_id: i32,
    quantity: i32,
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<Discount>, diesel::result::Error> {
    crate::data::schema::discounts::table
        .inner_join(
            crate::data::schema::discount_products::table
                .on(crate::data::schema::discount_products::discount_id
                    .eq(crate::data::schema::discounts::id)),
        )
        .filter(crate::data::schema::discount_products::product_id.eq(product_id))
        .filter(crate::data::schema::discounts::start_date.le(diesel::dsl::now))
        .filter(crate::data::schema::discounts::end_date.ge(diesel::dsl::now))
        .filter(crate::data::schema::discounts::min_quantity.le(quantity))
        .select(crate::data::schema::discounts::all_columns)
        .load::<Discount>(connection)
}
