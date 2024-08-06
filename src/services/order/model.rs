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
#[diesel(table_name = crate::schema::order_lines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
pub struct OrderLine {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub warehouse_id: i32,
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
fn default_warehouse_id() -> i32 {
    1
}
#[derive(
    Queryable,
    Selectable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Insertable,
    AsChangeset,
    Associations,
    Clone,
)]
#[diesel(table_name = crate::schema::order_lines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Cart))]
#[diesel(belongs_to(Product))]
pub struct NewOrderLine {
    pub cart_id: i32,
    pub product_id: i32,
    #[serde(default = "default_warehouse_id")]
    pub warehouse_id: i32,
    pub quantity: i32,
}
impl NewOrderLine {
    pub fn new(cart_id: i32, product_id: i32, warehouse_id: Option<i32>, quantity: i32) -> Self {
        Self {
            cart_id,
            product_id,
            warehouse_id: warehouse_id.or(Some(1)).unwrap(),
            quantity,
        }
    }
}
use crate::schema::warehouses;

#[derive(
    Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Insertable,
)]
#[diesel(table_name = warehouses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = warehouses)]
pub struct NewWarehouse {
    pub name: String,
}
