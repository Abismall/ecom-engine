use super::model::relations::DiscountBrand;
use super::model::relations::DiscountCategory;
use super::model::relations::DiscountProduct;
use super::model::Discount;
use super::model::NewDiscount;
use diesel::prelude::*;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub fn select_discount_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    discount_id: i32,
) -> Result<Discount, diesel::result::Error> {
    crate::data::schema::discounts::table
        .select(Discount::as_select())
        .filter(crate::data::schema::discounts::id.eq(discount_id))
        .first::<Discount>(connection)
}

pub fn delete_discount_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    discount_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(crate::data::schema::discounts::table.find(discount_id)).execute(connection)
}

pub fn load_discounts_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
) -> Result<Vec<Discount>, diesel::result::Error> {
    crate::data::schema::discounts::table.load::<Discount>(connection)
}

pub fn insert_discount_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    new_discount: NewDiscount,
) -> Result<Discount, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::discounts::table)
        .values(&new_discount)
        .returning(crate::data::schema::discounts::all_columns)
        .get_result::<Discount>(connection)
}

pub fn set_discount_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    updated_discount: Discount,
) -> Result<Discount, diesel::result::Error> {
    diesel::update(crate::data::schema::discounts::table.find(updated_discount.id))
        .set(&updated_discount)
        .get_result::<Discount>(connection)
}

pub fn insert_discount_brand_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    new_discount: DiscountBrand,
) -> Result<DiscountBrand, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::discount_brands::table)
        .values(new_discount)
        .get_result::<DiscountBrand>(connection)
}

pub fn insert_discount_category_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    new_discount: DiscountCategory,
) -> Result<DiscountCategory, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::discount_categories::table)
        .values(new_discount)
        .get_result::<DiscountCategory>(connection)
}

pub fn insert_discount_product_query(
    connection: &mut PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>,
    new_discount: DiscountProduct,
) -> Result<DiscountProduct, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::discount_products::table)
        .values(new_discount)
        .get_result::<DiscountProduct>(connection)
}
