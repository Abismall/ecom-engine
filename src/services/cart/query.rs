use crate::error::DatabaseErrorWrapper;
use crate::services::discount::utils::sort_discounts_by_start_date_desc;
use crate::services::order::query::load_orderlines_query;
use crate::services::order::utils::map_orderlines_to_carts;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use super::model::{Cart, CartWithOrderLines};

fn load_carts_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<Cart>, DatabaseErrorWrapper> {
    use crate::data::schema::carts::dsl::*;
    carts.load::<Cart>(connection).map_err(DatabaseErrorWrapper)
}

pub fn insert_cart_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Cart, DatabaseErrorWrapper> {
    diesel::insert_into(crate::data::schema::carts::table)
        .values(&Cart {
            id: chrono::Utc::now().timestamp() as i32,
            is_active: true,
        })
        .get_result::<Cart>(connection)
        .map_err(DatabaseErrorWrapper)
}
pub fn set_cart_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    updated_cart: Cart,
) -> Result<Cart, DatabaseErrorWrapper> {
    diesel::update(crate::data::schema::carts::table.find(updated_cart.id))
        .set(&updated_cart)
        .get_result::<Cart>(connection)
        .map_err(DatabaseErrorWrapper)
}
pub fn select_cart_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    discount_id: i32,
) -> Result<Cart, DatabaseErrorWrapper> {
    crate::data::schema::carts::table
        .select(Cart::as_select())
        .filter(crate::data::schema::carts::id.eq(discount_id))
        .first::<Cart>(connection)
        .map_err(DatabaseErrorWrapper)
}
pub fn delete_cart_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    cart_id: i32,
) -> Result<usize, DatabaseErrorWrapper> {
    diesel::delete(
        crate::data::schema::carts::table.filter(crate::data::schema::carts::id.eq(cart_id)),
    )
    .execute(connection)
    .map_err(DatabaseErrorWrapper)
}
pub async fn list_carts_with_orderlines_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<CartWithOrderLines>, DatabaseErrorWrapper> {
    let cart_vector = load_carts_query(connection)?;
    let orderline_in_cart_vector =
        load_orderlines_query(connection, sort_discounts_by_start_date_desc).unwrap();
    Ok(map_orderlines_to_carts(
        cart_vector,
        orderline_in_cart_vector,
    ))
}

// fn load_carts_query(connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>) -> Result<Vec<Cart>, AppError> {
//     use crate::data::schema::carts::dsl::*;
//     let carts_result = carts.load::<Cart>(connection)?;
//     Ok(carts_result)
// }

// pub fn list_carts_with_orderlines(
//     connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
// ) -> Result<Vec<CartWithOrderLines>, AppError> {
//     let cart_vector = load_carts_query(connection)?;
//     let orderline_in_cart_vector = load_orderlines_query(
//         connection,
//         crate::util::DiscountResolver::sort_discounts_by_start_date_desc,
//     )?;
//     Ok(crate::util::map_orderlines_to_carts(
//         cart_vector,
//         orderline_in_cart_vector,
//     ))
// }
