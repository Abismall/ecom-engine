use crate::services::discount::model::Discount;
use crate::services::product::model::Product;
use diesel::prelude::*;
use diesel::PgConnection;

use super::model::Category;
use super::model::NewCategory;

pub fn load_categories_query(
    connection: &mut PgConnection,
) -> Result<Vec<Category>, diesel::result::Error> {
    crate::data::schema::categories::table.load::<Category>(connection)
}

pub fn select_category_query(
    connection: &mut PgConnection,
    id: i32,
) -> Result<Category, diesel::result::Error> {
    crate::data::schema::categories::table
        .select(Category::as_select())
        .filter(crate::data::schema::categories::id.eq(id))
        .first::<Category>(connection)
}

pub fn inner_join_products_with_categories(
    connection: &mut PgConnection,
    category_id: i32,
) -> Result<Vec<(Product, Category)>, diesel::result::Error> {
    crate::data::schema::products::table
        .inner_join(crate::data::schema::categories::table)
        .filter(crate::data::schema::categories::id.eq(category_id))
        .select((Product::as_select(), Category::as_select()))
        .load::<(Product, Category)>(connection)
}

pub fn insert_product_query(
    connection: &mut PgConnection,
    new_category: NewCategory,
) -> Result<Category, diesel::result::Error> {
    diesel::insert_into(crate::data::schema::categories::table)
        .values(&new_category)
        .returning(crate::data::schema::categories::all_columns)
        .get_result::<Category>(connection)
}

pub fn set_product_query(
    connection: &mut PgConnection,
    updated_category: Category,
) -> Result<Category, diesel::result::Error> {
    diesel::update(crate::data::schema::categories::table.find(updated_category.id))
        .set(&updated_category)
        .get_result::<Category>(connection)
}

pub fn delete_product_query(
    connection: &mut PgConnection,
    category_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(
        crate::data::schema::categories::table
            .filter(crate::data::schema::categories::id.eq(category_id)),
    )
    .execute(connection)
}

pub fn fetch_category_discounts(
    requested_category_id: i32,
    quantity: i32,
    connection: &mut PgConnection,
) -> Result<Vec<Discount>, diesel::result::Error> {
    use crate::data::schema::discount_categories::dsl::*;
    use crate::data::schema::discounts::dsl::*;

    discounts
        .inner_join(
            discount_categories.on(crate::data::schema::discount_categories::discount_id
                .eq(crate::data::schema::discounts::id)),
        )
        .filter(crate::data::schema::discount_categories::category_id.eq(requested_category_id))
        .filter(crate::data::schema::discounts::start_date.le(chrono::Utc::now().naive_utc()))
        .filter(crate::data::schema::discounts::end_date.ge(chrono::Utc::now().naive_utc()))
        .filter(crate::data::schema::discounts::min_quantity.le(quantity))
        .select(crate::data::schema::discounts::all_columns)
        .load::<Discount>(connection)
}
