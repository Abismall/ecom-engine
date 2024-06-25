use crate::services::discount::model::break_down::Resolver;
use crate::services::product::model::Product;
use crate::{services::cart::model::Cart, services::discount::model::Discount};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Insertable,
    AsChangeset,
    Associations,
    Clone,
)]
#[diesel(table_name = crate::data::schema::order_lines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
pub struct OrderLine {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct OrderLineWithDiscounts {
    pub order_line: OrderLine,
    pub product: Product,
    pub discounts: Vec<Discount>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct OrderLineInCart {
    pub id: i32,
    pub cart_id: i32,
    pub product: Product,
    pub quantity: i32,
    pub orderline_total: i32,
    pub orderline_total_discount_amount: i32,
    pub orderline_total_with_discount: i32,
    pub discounts: Vec<Discount>,
    pub discount_resolution_breakdown: Resolver,
}

impl OrderLineInCart {
    pub fn new(
        id: i32,
        cart_id: i32,
        product: Product,
        quantity: i32,
        orderline_total: i32,
        orderline_total_discount_amount: i32,
        orderline_total_with_discount: i32,
        discounts: Vec<Discount>,
    ) -> Self {
        OrderLineInCart {
            id,
            cart_id,
            product,
            quantity,
            orderline_total,
            orderline_total_discount_amount,
            orderline_total_with_discount,
            discounts,
            discount_resolution_breakdown: Resolver::default(),
        }
    }
}

#[derive(Queryable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::data::schema::order_lines)]
pub struct NewOrderLine {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

use crate::data::schema::warehouses;

#[derive(
    Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Insertable,
)]
#[diesel(table_name = warehouses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = warehouses)]
pub struct NewWarehouse<'a> {
    pub name: &'a String,
}
