use crate::data::schema::stock_quantities;
use crate::services::order::model::Warehouse;
use crate::services::product::model::Product;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[derive(
    Queryable,
    Selectable,
    Identifiable,
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
    pub id: i32,
    pub warehouse_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Insertable)]
#[diesel(table_name = stock_quantities)]
pub struct NewStockQuantity<'a> {
    pub product_id: &'a i32,
    pub warehouse_id: &'a i32,
    pub quantity: &'a i32,
}
