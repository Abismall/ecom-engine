use crate::postgres::PooledConnection;
use crate::services::discount::utils::sort_discounts_by_start_date_desc;
use crate::services::order::query::load_orderlines_query;
use crate::services::order::utils::map_orderlines_to_carts;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

use super::model::{Cart, CartWithOrderLines};

fn load_carts_query(connection: &mut PooledConnection) -> Result<Vec<Cart>, diesel::result::Error> {
    use crate::schema::carts::dsl::*;
    carts.load::<Cart>(connection)
}

pub fn insert_cart_query(connection: &mut PooledConnection) -> Result<Cart, diesel::result::Error> {
    diesel::insert_into(crate::schema::carts::table)
        .values(&Cart {
            id: chrono::Utc::now().timestamp() as i32,
            is_active: true,
        })
        .get_result::<Cart>(connection)
}
pub fn set_cart_query(
    connection: &mut PooledConnection,
    updated_cart: Cart,
) -> Result<Cart, diesel::result::Error> {
    diesel::update(crate::schema::carts::table.find(updated_cart.id))
        .set(&updated_cart)
        .get_result::<Cart>(connection)
}
pub fn select_cart_query(
    connection: &mut PooledConnection,
    discount_id: i32,
) -> Result<Cart, diesel::result::Error> {
    crate::schema::carts::table
        .select(Cart::as_select())
        .filter(crate::schema::carts::id.eq(discount_id))
        .first::<Cart>(connection)
}
pub fn delete_cart_query(
    connection: &mut PooledConnection,
    cart_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(crate::schema::carts::table.filter(crate::schema::carts::id.eq(cart_id)))
        .execute(connection)
}
pub async fn list_carts_with_orderlines_query(
    connection: &mut PooledConnection,
) -> Result<Vec<CartWithOrderLines>, diesel::result::Error> {
    let cart_vector = load_carts_query(connection)?;
    let orderline_in_cart_vector =
        load_orderlines_query(connection, sort_discounts_by_start_date_desc).unwrap();
    Ok(map_orderlines_to_carts(
        cart_vector,
        orderline_in_cart_vector,
    ))
}

// fn load_carts_query(connection: &mut PooledConnection) -> Result<Vec<Cart>, AppError> {
//     use crate::schema::carts::dsl::*;
//     let carts_result = carts.load::<Cart>(connection)?;
//     Ok(carts_result)
// }

// pub fn list_carts_with_orderlines(
//     connection: &mut PooledConnection,
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
