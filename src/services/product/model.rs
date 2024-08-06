use crate::schema::products;
use crate::services::brand::model::Brand;
use crate::services::{category::model::Category, discount::model::Discount};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = attributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Product))]
pub struct Attribute {
    pub id: i32,
    pub name: String,
    pub value: String,
}
#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = product_attributes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Attribute))]
pub struct ProductAttribute {
    pub product_id: i32,
    pub attribute_id: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultAttributes {
    pub size: Option<String>,
    pub color: Option<String>,
    pub weight: Option<i32>,
    pub weight_unit: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ProductWithAttributes {
    pub product: Product,
    pub attributes: DefaultAttributes,
    pub stock_quantity: i32,
}
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
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Category))]
#[diesel(belongs_to(Brand))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub in_stock: bool,
    pub category_id: Option<i32>,
    pub brand_id: Option<i32>,
    pub price: i32,
    pub tax_rate: i32,
}


#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: Option<String>,
    pub in_stock: Option<bool>,
    pub category_id: Option<i32>,
    pub brand_id: Option<i32>,
    pub price: Option<i32>,
    pub tax_rate: Option<i32>,
}

impl NewProduct {
    pub fn new(
        name: Option<String>,
        in_stock: Option<bool>,

        category_id: Option<i32>,
        brand_id: Option<i32>,
        price: Option<i32>,
        tax_rate: Option<i32>,
    ) -> Self {
        Self {
            name: name.or_else(|| Some("Default".to_string())),
            in_stock: in_stock.or(Some(true)),
            category_id: category_id.or(None),
            brand_id: brand_id.or(None),
            price: price.or(Some(0)),
            tax_rate: tax_rate.or(Some(0)),
        }
    }
}

// Builder pattern for creating NewProduct
#[derive(Debug, Default)]
pub struct ProductBuilder {
    id: i32,
    name: String,
    price: Option<i32>,
    category_id: Option<i32>,
    tax_rate: Option<i32>,
    in_stock: Option<bool>,
    size: Option<String>,
    color: Option<String>,
    weight: Option<i32>,
    weight_unit: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    brand_id: Option<i32>,
}

impl ProductBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn price(&mut self, price: i32) -> &mut Self {
        self.price = Some(price);
        self
    }

    pub fn category_id(&mut self, category_id: i32) -> &mut Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn tax_rate(&mut self, tax_rate: i32) -> &mut Self {
        self.tax_rate = Some(tax_rate);
        self
    }

    pub fn in_stock(&mut self, in_stock: bool) -> &mut Self {
        self.in_stock = Some(in_stock);
        self
    }

    pub fn size(&mut self, size: String) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn color(&mut self, color: String) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn weight(&mut self, weight: i32) -> &mut Self {
        self.weight = Some(weight);
        self
    }

    pub fn weight_unit(&mut self, weight_unit: String) -> &mut Self {
        self.weight_unit = Some(weight_unit);
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn brand_id(&mut self, brand_id: i32) -> &mut Self {
        self.brand_id = Some(brand_id);
        self
    }

    pub fn build(&self) -> Product {
        Product {
            id: self.id.clone(),
            name: self.name.clone(),
            in_stock: *self.in_stock.as_ref().unwrap_or(&false),

            category_id: None,
            brand_id: None,
            price: *self.price.as_ref().unwrap_or(&0),
            tax_rate: *self.tax_rate.as_ref().unwrap_or(&0),
        }
    }
}

impl Product {
    pub fn new(name: String) -> ProductBuilder {
        ProductBuilder::new(name)
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct ProductWithDiscount {
    pub product: Product,
    pub discounts: Vec<Discount>,
}
