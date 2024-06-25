use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use r2d2::PooledConnection;

use crate::services::{discount::model::{relations::DiscountBrand, Discount}, product::model::Product};

use super::model::{Brand, NewBrand};

pub fn load_brand_products_query(
    connection: &mut PgConnection,
    brand_id: i32,
) -> Result<Vec<(Product, Brand)>, diesel::result::Error> {
    crate::data::schema::products::table
        .inner_join(crate::data::schema::brands::table)
        .filter(crate::data::schema::brands::id.eq(brand_id))
        .select((Product::as_select(), Brand::as_select()))
        .load::<(Product, Brand)>(connection)
}

// Insert a new brand
pub fn insert_brand_query(
    connection: &mut PgConnection,
    new_brand: NewBrand,
) -> Result<Brand, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::brands::table)
        .values(&new_brand)
        .returning(Brand::as_select())
        .get_result::<Brand>(connection)
}

// Update an existing brand
pub fn set_brand_query(
    connection: &mut PgConnection,
    updated_brand: Brand,
) -> Result<Brand, diesel::result::Error> {
    diesel::update(crate::data::schema::brands::table.find(updated_brand.id))
        .set(&updated_brand)
        .get_result::<Brand>(connection)
}

pub async fn select_brand_query(
    connection: &mut PgConnection,
    brand_id: i32,
) -> Result<Brand, diesel::result::Error> {
    crate::data::schema::brands::table
        .select(Brand::as_select())
        .filter(crate::data::schema::brands::id.eq(brand_id))
        .first::<Brand>(connection)
}

// Delete a brand by ID
pub fn delete_brand_query(
    connection: &mut PgConnection,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(crate::data::schema::brands::table.find(id)).execute(connection)
}

// Load all brands
pub fn load_brand_query(
    connection: &mut PgConnection,
) -> Result<Vec<Brand>, diesel::result::Error> {
    crate::data::schema::brands::table.load::<Brand>(connection)
}

pub fn fetch_brand_discounts(
    requested_brand_id: i32,
    quantity: i32,
    connection: &mut PgConnection,
) -> Result<Vec<Discount>, diesel::result::Error> {
    use crate::data::schema::discount_brands::dsl::*;
    use crate::data::schema::discounts::dsl::*;

    discounts
        .inner_join(diesel::JoinOnDsl::on(
            discount_brands,
            crate::data::schema::discount_brands::discount_id
                .eq(crate::data::schema::discounts::id),
        ))
        .filter(crate::data::schema::discount_brands::brand_id.eq(requested_brand_id))
        .filter(crate::data::schema::discounts::start_date.le(chrono::Utc::now().naive_utc()))
        .filter(crate::data::schema::discounts::end_date.ge(chrono::Utc::now().naive_utc()))
        .filter(crate::data::schema::discounts::min_quantity.le(quantity))
        .select(crate::data::schema::discounts::all_columns)
        .load::<Discount>(connection)
}
pub fn insert_discount_brand_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    new_discount: DiscountBrand,
) -> Result<DiscountBrand, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::discount_brands::table)
        .values(new_discount)
        .get_result::<DiscountBrand>(connection)
}
