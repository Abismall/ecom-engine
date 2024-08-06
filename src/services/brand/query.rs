use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    postgres::PooledConnection,
    services::{
        discount::model::{relations::DiscountBrand, Discount},
        product::model::Product,
    },
};

use super::model::{Brand, NewBrand};

pub fn load_brand_products_query(
    connection: &mut PooledConnection,
    brand_id: i32,
) -> Result<Vec<(Product, Brand)>, diesel::result::Error> {
    crate::schema::products::table
        .inner_join(crate::schema::brands::table)
        .filter(crate::schema::brands::id.eq(brand_id))
        .select((Product::as_select(), Brand::as_select()))
        .load::<(Product, Brand)>(connection)
}

// Insert a new brand
pub fn insert_brand_query(
    connection: &mut PooledConnection,
    new_brand: NewBrand,
) -> Result<Brand, diesel::result::Error> {
    diesel::insert_into(crate::schema::brands::table)
        .values(&new_brand)
        .returning(Brand::as_select())
        .get_result::<Brand>(connection)
}

// Update an existing brand
pub fn set_brand_query(
    connection: &mut PooledConnection,
    updated_brand: Brand,
) -> Result<Brand, diesel::result::Error> {
    diesel::update(crate::schema::brands::table.find(updated_brand.id))
        .set(&updated_brand)
        .get_result::<Brand>(connection)
}

pub async fn select_brand_query(
    connection: &mut PooledConnection,
    brand_id: i32,
) -> Result<Brand, diesel::result::Error> {
    crate::schema::brands::table
        .select(Brand::as_select())
        .filter(crate::schema::brands::id.eq(brand_id))
        .first::<Brand>(connection)
}

// Delete a brand by ID
pub fn delete_brand_query(
    connection: &mut PooledConnection,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(crate::schema::brands::table.find(id)).execute(connection)
}

// Load all brands
pub fn load_brand_query(
    connection: &mut PooledConnection,
) -> Result<Vec<Brand>, diesel::result::Error> {
    crate::schema::brands::table.load::<Brand>(connection)
}

pub fn fetch_brand_discounts(
    requested_brand_id: i32,
    quantity: i32,
    connection: &mut PooledConnection,
) -> Result<Vec<Discount>, diesel::result::Error> {
    use crate::schema::discount_brands::dsl::*;
    use crate::schema::discounts::dsl::*;

    discounts
        .inner_join(diesel::JoinOnDsl::on(
            discount_brands,
            crate::schema::discount_brands::discount_id.eq(crate::schema::discounts::id),
        ))
        .filter(crate::schema::discount_brands::brand_id.eq(requested_brand_id))
        .filter(crate::schema::discounts::start_date.le(chrono::Utc::now().naive_utc()))
        .filter(crate::schema::discounts::end_date.ge(chrono::Utc::now().naive_utc()))
        .filter(crate::schema::discounts::min_quantity.le(quantity))
        .select(crate::schema::discounts::all_columns)
        .load::<Discount>(connection)
}
pub fn insert_discount_brand_query(
    connection: &mut PooledConnection,
    new_discount: DiscountBrand,
) -> Result<DiscountBrand, diesel::result::Error> {
    diesel::insert_into(crate::schema::discount_brands::table)
        .values(new_discount)
        .get_result::<DiscountBrand>(connection)
}
