use crate::{schema::carts, services::order::model::OrderLineInCart};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
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
    Clone,
    AsChangeset,
)]
#[diesel(table_name = carts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cart {
    pub id: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartWithOrderLines {
    pub cart: Cart,
    pub order_lines: Vec<OrderLineInCart>,
}
