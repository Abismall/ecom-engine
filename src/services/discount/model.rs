use crate::schema::discounts;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    Clone,
)]
#[diesel(table_name = discounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Brand))]
#[diesel(belongs_to(Category))]
#[diesel(belongs_to(Product))]
pub struct Discount {
    pub id: i32,
    pub name: String,
    pub discount_type: String,
    pub value: i32,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: chrono::NaiveDateTime,
    pub min_quantity: i32,
}


#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = discounts)]
pub struct NewDiscount {
    pub name: String,
    pub discount_type: String,
    pub value: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub min_quantity: i32,
}

pub mod relations {
    use crate::schema::{discount_brands, discount_categories, discount_products};
    use crate::services::brand::model::Brand;
    use crate::services::category::model::Category;
    use crate::services::discount::model::Discount;
    use crate::services::product::model::Product;
    use diesel::{associations::Identifiable, Associations, Insertable, Queryable, Selectable};
    use serde::{Deserialize, Serialize};
    #[derive(
        Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize, Insertable,
    )]
    #[diesel(belongs_to(Discount))]
    #[diesel(belongs_to(Brand))]
    #[diesel(table_name = discount_brands)]
    #[diesel(primary_key(discount_id, brand_id))]
    pub struct DiscountBrand {
        pub discount_id: i32,
        pub brand_id: i32,
    }

    #[derive(
        Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize, Insertable,
    )]
    #[diesel(belongs_to(Discount))]
    #[diesel(belongs_to(Category))]
    #[diesel(table_name = discount_categories)]
    #[diesel(primary_key(discount_id, category_id))]
    pub struct DiscountCategory {
        pub discount_id: i32,
        pub category_id: i32,
    }

    #[derive(
        Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize, Insertable,
    )]
    #[diesel(belongs_to(Discount))]
    #[diesel(belongs_to(Product))]
    #[diesel(table_name = discount_products)]
    #[diesel(primary_key(discount_id, product_id))]
    pub struct DiscountProduct {
        pub discount_id: i32,
        pub product_id: i32,
    }
}

pub mod break_down {
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Resolver {
        pub steps: Vec<Value>,
    }

    impl Resolver {
        pub fn new() -> Self {
            Resolver { steps: Vec::new() }
        }

        pub fn add_step(
            &mut self,
            description: &str,
            orderline_total: i32,
            discount_value: i32,
            new_total: i32,
        ) {
            let step = json!({
                "description": description,
                "orderline_total": orderline_total,
                "discount_value": discount_value,
                "new_total": new_total,
            });
            self.steps.push(step);
        }

        pub fn to_string(&self) -> String {
            serde_json::to_string_pretty(&self.steps).unwrap()
        }
    }

    impl Default for Resolver {
        fn default() -> Self {
            Resolver::new()
        }
    }
}
