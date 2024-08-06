use crate::schema::brands;
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
    AsChangeset,
)]
#[diesel(table_name = brands)]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = brands)]
pub struct NewBrand {
    pub name: String,
    pub description: Option<String>,
}

// API Requests for Brand
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBrandRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBrandRequest {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductsForBrandInCategoryRequest {
    pub id: i32,
    pub category_id: i32,
}
