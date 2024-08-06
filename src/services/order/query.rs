use crate::postgres::PooledConnection;
use crate::services::brand::query::fetch_brand_discounts;
use crate::services::category::query::fetch_category_discounts;
use crate::services::discount::model::break_down;
use crate::services::discount::model::Discount;
use crate::services::discount::utils::calculate_orderline_total;
use crate::services::product::model::Product;
use crate::services::product::query::fetch_product_discounts_query;

use diesel::prelude::*;
use diesel::{RunQueryDsl, SelectableHelper};

use super::model::NewOrderLine;
use super::model::OrderLine;
use super::model::OrderLineInCart;
use super::utils::calculate_orderline_total_discount;
use super::utils::calculate_total_with_discount;
pub fn insert_orderline_query(
    connection: &mut PooledConnection,
    new_orderline: NewOrderLine,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::order_lines::dsl::*;

    let orderline = NewOrderLine::new(
        new_orderline.cart_id,
        new_orderline.product_id,
        Some(new_orderline.warehouse_id),
        new_orderline.quantity,
    );

    diesel::insert_into(order_lines)
        .values(&orderline)
        .on_conflict((cart_id, product_id))
        .do_update()
        .set(quantity.eq(quantity + new_orderline.quantity))
        .execute(connection)
}
pub fn select_orderline_query(
    connection: &mut PooledConnection,
    orderline_id: i32,
) -> Result<OrderLine, diesel::result::Error> {
    crate::schema::order_lines::table
        .select(OrderLine::as_select())
        .filter(crate::schema::order_lines::id.eq(orderline_id))
        .first::<OrderLine>(connection)
}
pub fn set_orderline_query(
    connection: &mut PooledConnection,
    updated_orderline: OrderLine,
) -> Result<OrderLine, diesel::result::Error> {
    diesel::update(crate::schema::order_lines::table.find(updated_orderline.id))
        .set(&updated_orderline)
        .get_result::<OrderLine>(connection)
}
pub fn delete_orderline_query(
    connection: &mut PooledConnection,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(crate::schema::order_lines::table.find(id)).execute(connection)
}
pub fn load_orderlines_query(
    connection: &mut PooledConnection,
    discount_sort_fn: fn(&mut Vec<Discount>),
) -> Result<Vec<OrderLineInCart>, diesel::result::Error> {
    let order_lines_with_products: Vec<(OrderLine, Product)> = crate::schema::order_lines::table
        .inner_join(crate::schema::products::table)
        .select((
            crate::schema::order_lines::all_columns,
            crate::schema::products::all_columns,
        ))
        .load::<(OrderLine, Product)>(connection)?;

    let mut results: Vec<OrderLineInCart> = Vec::new();

    for (order_line, product) in order_lines_with_products {
        let mut all_discounts =
            fetch_all_discounts_for_orderline(&order_line, &product, connection, discount_sort_fn)?;

        discount_sort_fn(&mut all_discounts);

        let orderline_total = calculate_orderline_total(order_line.quantity, product.price);
        let orderline_total_discount_amount =
            calculate_orderline_total_discount(orderline_total, &all_discounts);
        let orderline_total_with_discount =
            calculate_total_with_discount(orderline_total, orderline_total_discount_amount);

        let order_line_in_cart = OrderLineInCart {
            id: order_line.id,
            cart_id: order_line.cart_id,
            product,
            quantity: order_line.quantity,
            orderline_total,
            orderline_total_discount_amount,
            orderline_total_with_discount,
            discounts: all_discounts,
            discount_resolution_breakdown: break_down::Resolver::new(),
        };

        results.push(order_line_in_cart);
    }

    Ok(results)
}

fn fetch_all_discounts_for_orderline(
    order_line: &OrderLine,
    product: &Product,
    connection: &mut PooledConnection,
    sort_fn: fn(&mut Vec<Discount>),
) -> Result<Vec<Discount>, diesel::result::Error> {
    let product_discounts =
        fetch_product_discounts_query(product.id, order_line.quantity, connection)?;
    let brand_discounts = match product.brand_id {
        Some(brand_id) => fetch_brand_discounts(brand_id, order_line.quantity, connection)?,
        None => Vec::<Discount>::new(),
    };
    let category_discounts = match product.category_id {
        Some(category_id) => {
            fetch_category_discounts(category_id, order_line.quantity, connection)?
        }
        None => Vec::<Discount>::new(),
    };

    let mut all_discounts = product_discounts;
    all_discounts.extend(brand_discounts);
    all_discounts.extend(category_discounts);

    // Remove duplicate discounts
    all_discounts.sort_by(|a, b| a.id.cmp(&b.id));
    all_discounts.dedup_by(|a, b| a.id == b.id);

    // Sort discounts using the provided sorting function
    sort_fn(&mut all_discounts);

    Ok(all_discounts)
}
