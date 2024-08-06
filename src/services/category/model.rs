use crate::schema::categories;
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
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Category {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a String,
    pub description: &'a String,
}

impl<'a> NewCategory<'a> {
    pub fn new(name: &'a String, description: &'a String) -> Self {
        Self { name, description }
    }
}

// API Requests for Category
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryRequest {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCategoryRequest {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetProductByCategory {
    pub id: i32,
}
