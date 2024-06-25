use std::collections::HashMap;

use crate::{
    services::cart::model::{Cart, CartWithOrderLines},
    services::discount::{
        model::{break_down::Resolver, Discount},
        utils::calculate_discount_amount,
    },
};

use super::model::OrderLineInCart;
pub fn update_existing_order_line(
    existing_order_line: &mut OrderLineInCart,
    new_order_line: &OrderLineInCart,
) {
    existing_order_line.quantity += new_order_line.quantity;
    existing_order_line.orderline_total += new_order_line.quantity * new_order_line.product.price;
    existing_order_line
        .discounts
        .extend(new_order_line.discounts.clone());

    let (total_discount, breakdown) = calculate_total_discount(
        existing_order_line.orderline_total,
        &existing_order_line.discounts,
    );
    existing_order_line.orderline_total_discount_amount = total_discount;
    existing_order_line.orderline_total_with_discount =
        existing_order_line.orderline_total - total_discount;
    existing_order_line.discount_resolution_breakdown = breakdown;
}

pub fn create_new_order_line(order_line_with_discounts: &OrderLineInCart) -> OrderLineInCart {
    let orderline_total =
        order_line_with_discounts.quantity * order_line_with_discounts.product.price;

    let (orderline_total_discount_amount, breakdown) =
        calculate_total_discount(orderline_total, &order_line_with_discounts.discounts);

    let orderline_total_with_discount = orderline_total - orderline_total_discount_amount;

    OrderLineInCart {
        orderline_total,
        id: order_line_with_discounts.id,
        cart_id: order_line_with_discounts.cart_id,
        product: order_line_with_discounts.product.clone(),
        quantity: order_line_with_discounts.quantity,
        discounts: order_line_with_discounts.discounts.clone(),
        orderline_total_discount_amount,
        orderline_total_with_discount,
        discount_resolution_breakdown: breakdown,
    }
}
pub fn calculate_total_with_discount(orderline_total: i32, total_discount: i32) -> i32 {
    orderline_total - total_discount
}
pub fn calculate_orderline_total_discount(orderline_total: i32, discounts: &[Discount]) -> i32 {
    discounts
        .iter()
        .map(|discount| calculate_discount_amount(orderline_total, discount))
        .sum()
}

pub fn calculate_total_discount(orderline_total: i32, discounts: &[Discount]) -> (i32, Resolver) {
    let mut total_discount = 0;
    let mut breakdown = Resolver::new();
    let mut current_price = orderline_total;

    // Ensure discounts are processed in a consistent order
    let mut sorted_discounts = discounts.to_vec();
    sorted_discounts.sort_by(|a, b| a.id.cmp(&b.id));

    for discount in sorted_discounts {
        let discount_amount = match discount.discount_type.as_str() {
            "percentage" => {
                let discount_value =
                    (current_price as f32 * (discount.value as f32 / 100.0)).round() as i32;
                breakdown.add_step(
                    &format!(
                        "{} (ID: {}): {}% off",
                        discount.name, discount.id, discount.value
                    ),
                    current_price,
                    discount_value,
                    current_price - discount_value,
                );
                discount_value
            }
            "fixed" => {
                breakdown.add_step(
                    &format!(
                        "{} (ID: {}): ${} off",
                        discount.name, discount.id, discount.value
                    ),
                    current_price,
                    discount.value,
                    current_price - discount.value,
                );
                discount.value
            }
            _ => 0,
        };
        total_discount += discount_amount;
        current_price -= discount_amount;
    }

    (total_discount, breakdown)
}

pub fn map_orderlines_to_carts(
    cart_vector: Vec<Cart>,
    order_lines_with_discounts_vector: Vec<OrderLineInCart>,
) -> Vec<CartWithOrderLines> {
    let mut carts_map: HashMap<i32, CartWithOrderLines> = HashMap::new();

    // Initialize carts with empty order lines
    for cart in cart_vector {
        carts_map.insert(
            cart.id,
            CartWithOrderLines {
                cart,
                order_lines: Vec::new(),
            },
        );
    }

    // Aggregate order lines by product ID within each cart
    let mut order_lines_map: HashMap<(i32, i32), OrderLineInCart> = HashMap::new();

    for order_line_with_discounts in order_lines_with_discounts_vector {
        let key = (
            order_line_with_discounts.cart_id,
            order_line_with_discounts.product.id,
        );

        if let Some(existing_order_line) = order_lines_map.get_mut(&key) {
            update_existing_order_line(existing_order_line, &order_line_with_discounts);
        } else {
            let new_order_line = create_new_order_line(&order_line_with_discounts);
            order_lines_map.insert(key, new_order_line);
        }
    }

    // Populate the order lines into carts
    for order_line_in_cart in order_lines_map.values() {
        if let Some(cart_with_order_lines) = carts_map.get_mut(&order_line_in_cart.cart_id) {
            cart_with_order_lines
                .order_lines
                .push(order_line_in_cart.clone());
        }
    }

    carts_map.into_iter().map(|(_, v)| v).collect()
}
