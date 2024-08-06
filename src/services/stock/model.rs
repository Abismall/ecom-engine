use crate::schema::stock_quantities;
use crate::services::order::model::Warehouse;
use crate::services::product::model::Product;
use diesel::{AsChangeset, Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[derive(
    Queryable,
    Selectable,
    Associations,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = stock_quantities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Product))]
#[diesel(belongs_to(Warehouse))]
pub struct StockQuantity {
    pub warehouse_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = stock_quantities)]

pub struct NewStockQuantity {
    pub product_id: i32,
    pub warehouse_id: Option<i32>,
    pub quantity: i32,
}

impl NewStockQuantity {
    pub fn new(product_id: i32, quantity: i32, warehouse_id: Option<i32>) -> Self {
        Self {
            product_id,
            quantity,
            warehouse_id: Some(warehouse_id.or(Some(1)).unwrap()),
        }
    }
}
