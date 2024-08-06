use std::collections::HashMap;

use crate::error::DatabaseErrorWrapper;
use crate::postgres::PooledConnection;

use crate::services::discount::model::Discount;
use crate::services::stock::model::StockQuantity;

use diesel::prelude::*;
use diesel::PgConnection;

use super::model::{
    DefaultAttributes, NewProduct, Product, ProductWithAttributes, ProductWithDiscount,
};
use crate::services::product::model::Attribute;

pub fn insert_product_query_pooled_conn_internal<C>(
    new_product: NewProduct,
    connection: &mut C,
) -> Result<Product, DatabaseErrorWrapper>
where
    C: Connection<Backend = diesel::pg::Pg> + diesel::connection::LoadConnection,
{
    use crate::schema::products;

    diesel::insert_into(products::table)
        .values(&new_product)
        .returning(products::all_columns)
        .get_result::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn insert_product_query_pooled_conn(
    new_product: NewProduct,
    connection: &mut PooledConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    insert_product_query_pooled_conn_internal(new_product, connection)
}

pub fn insert_product_query(
    new_product: NewProduct,
    connection: &mut PgConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    insert_product_query_pooled_conn_internal(new_product, connection)
}

pub fn select_product_query(
    product_id: i32,
    connection: &mut PooledConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    use crate::schema::products::dsl::*;

    products
        .filter(id.eq(product_id))
        .first::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn inner_join_product_and_stock_query(
    connection: &mut PooledConnection,
) -> Result<Vec<(Product, StockQuantity)>, DatabaseErrorWrapper> {
    use crate::schema::{products, stock_quantities};

    products::table
        .inner_join(stock_quantities::table)
        .select((products::all_columns, stock_quantities::all_columns))
        .load::<(Product, StockQuantity)>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn set_product_query_pooled_conn(
    updated_product: Product,
    connection: &mut PooledConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    set_product_query_internal(updated_product, connection)
}

pub fn set_product_query(
    updated_product: Product,
    connection: &mut PgConnection,
) -> Result<Product, DatabaseErrorWrapper> {
    set_product_query_internal(updated_product, connection)
}

pub fn set_product_query_internal<C>(
    updated_product: Product,
    connection: &mut C,
) -> Result<Product, DatabaseErrorWrapper>
where
    C: Connection<Backend = diesel::pg::Pg> + diesel::connection::LoadConnection,
{
    use crate::schema::products;

    diesel::update(products::table.find(updated_product.id))
        .set(&updated_product)
        .get_result::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn delete_product_query(
    connection: &mut PooledConnection,
    product_id: i32,
) -> Result<usize, DatabaseErrorWrapper> {
    use crate::schema::products::dsl::products;

    diesel::delete(products.find(product_id))
        .execute(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn load_products_query(
    connection: &mut PooledConnection,
) -> Result<Vec<Product>, DatabaseErrorWrapper> {
    use crate::schema::products::dsl::products;

    products
        .load::<Product>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn load_products_with_attributes_query(
    connection: &mut PooledConnection,
) -> Result<Vec<ProductWithAttributes>, DatabaseErrorWrapper> {
    use crate::schema::{attributes, product_attributes, products};

    let product_with_attrs = products::table
        .left_join(product_attributes::table.on(products::id.eq(product_attributes::product_id)))
        .left_join(attributes::table.on(product_attributes::attribute_id.eq(attributes::id)))
        .select((products::all_columns, attributes::all_columns.nullable()))
        .load::<(Product, Option<Attribute>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Group attributes by product
    let mut products_map = HashMap::new();
    for (product, attribute_opt) in product_with_attrs {
        let entry = products_map
            .entry(product.id)
            .or_insert_with(|| ProductWithAttributes {
                product: product.clone(),
                attributes: DefaultAttributes::default(),
                stock_quantity: 0,
            });

        if let Some(attribute) = attribute_opt {
            match attribute.name.as_str() {
                "size" => entry.attributes.size = Some(attribute.value),
                "color" => entry.attributes.color = Some(attribute.value),
                "weight" => entry.attributes.weight = attribute.value.parse().ok(),
                "weight_unit" => entry.attributes.weight_unit = Some(attribute.value),
                "width" => entry.attributes.width = attribute.value.parse().ok(),
                "height" => entry.attributes.height = attribute.value.parse().ok(),
                _ => {}
            }
        }
    }

    Ok(products_map.into_iter().map(|(_, v)| v).collect())
}
fn product_discount(
    connection: &mut PooledConnection,
) -> Result<Vec<(Product, Option<Discount>)>, DatabaseErrorWrapper> {
    use crate::schema::{discount_products, discounts, products};
    use diesel::prelude::*;

    products::table
        .left_join(discount_products::table.on(products::id.eq(discount_products::product_id)))
        .left_join(
            discounts::table.on(discount_products::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)
}

fn product_brand_discount(
    connection: &mut PooledConnection,
) -> Result<Vec<(Product, Option<Discount>)>, DatabaseErrorWrapper> {
    use crate::schema::{discount_brands, discounts, products};
    use diesel::prelude::*;

    products::table
        .left_join(
            discount_brands::table.on(products::brand_id.eq(discount_brands::brand_id.nullable())),
        )
        .left_join(
            discounts::table.on(discount_brands::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)
}

fn product_category_discount(
    connection: &mut PooledConnection,
) -> Result<Vec<(Product, Option<Discount>)>, DatabaseErrorWrapper> {
    use crate::schema::{discount_categories, discounts, products};
    use diesel::prelude::*;

    products::table
        .left_join(
            discount_categories::table
                .on(products::category_id.eq(discount_categories::category_id.nullable())),
        )
        .left_join(
            discounts::table.on(discount_categories::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)
}
fn products_with_attributes(
    connection: &mut PooledConnection,
) -> Result<HashMap<i32, ProductWithAttributes>, DatabaseErrorWrapper> {
    use crate::schema::{attributes, product_attributes, products};
    // Fetch products with attributes
    let product_with_attrs: Vec<(Product, Option<Attribute>)> = products::table
        .left_join(product_attributes::table.on(products::id.eq(product_attributes::product_id)))
        .left_join(attributes::table.on(product_attributes::attribute_id.eq(attributes::id)))
        .select((products::all_columns, attributes::all_columns.nullable()))
        .load::<(Product, Option<Attribute>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Group attributes by product
    let mut products_map = HashMap::new();
    for (product, attribute_opt) in product_with_attrs {
        let entry = products_map
            .entry(product.id)
            .or_insert_with(|| ProductWithAttributes {
                product: product.clone(),
                attributes: DefaultAttributes::default(),
                stock_quantity: 0, // Initialize with zero
            });

        if let Some(attribute) = attribute_opt {
            match attribute.name.as_str() {
                "size" => entry.attributes.size = Some(attribute.value),
                "color" => entry.attributes.color = Some(attribute.value),
                "weight" => entry.attributes.weight = attribute.value.parse().ok(),
                "weight_unit" => entry.attributes.weight_unit = Some(attribute.value),
                "width" => entry.attributes.width = attribute.value.parse().ok(),
                "height" => entry.attributes.height = attribute.value.parse().ok(),
                _ => {}
            }
        }
    }
    Ok(products_map)
}
pub fn load_products_with_attributes_and_discounts_query(
    connection: &mut PooledConnection,
) -> Result<Vec<ProductWithAttributes>, DatabaseErrorWrapper> {
    use crate::schema::{
        attributes, discount_brands, discount_categories, discount_products, discounts,
        product_attributes, products, stock_quantities,
    };
    use diesel::prelude::*;

    // Fetch products with discounts from discount_products
    let product_discount_results = products::table
        .left_join(discount_products::table.on(products::id.eq(discount_products::product_id)))
        .left_join(
            discounts::table.on(discount_products::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Fetch products with discounts from discount_brands
    let product_brand_discount_results = products::table
        .left_join(
            discount_brands::table.on(products::brand_id.eq(discount_brands::brand_id.nullable())),
        )
        .left_join(
            discounts::table.on(discount_brands::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Fetch products with discounts from discount_categories
    let product_category_discount_results = products::table
        .left_join(
            discount_categories::table
                .on(products::category_id.eq(discount_categories::category_id.nullable())),
        )
        .left_join(
            discounts::table.on(discount_categories::discount_id
                .nullable()
                .eq(discounts::id.nullable())),
        )
        .select((products::all_columns, discounts::all_columns.nullable()))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Combine all discount results
    let all_results = product_discount_results
        .into_iter()
        .chain(product_brand_discount_results.into_iter())
        .chain(product_category_discount_results.into_iter())
        .collect::<Vec<_>>();

    let mut product_map: HashMap<i32, ProductWithDiscount> = HashMap::new();

    for (product, discount) in all_results {
        let product_id = product.id;
        product_map
            .entry(product_id)
            .and_modify(|entry| {
                if let Some(discount) = &discount {
                    entry.discounts.push(discount.clone());
                }
            })
            .or_insert_with(|| ProductWithDiscount {
                product,
                discounts: discount.into_iter().collect(),
            });
    }

    // Fetch products with attributes
    let product_with_attrs = products::table
        .left_join(product_attributes::table.on(products::id.eq(product_attributes::product_id)))
        .left_join(attributes::table.on(product_attributes::attribute_id.eq(attributes::id)))
        .select((products::all_columns, attributes::all_columns.nullable()))
        .load::<(Product, Option<Attribute>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Group attributes by product
    let mut products_map = HashMap::new();
    for (product, attribute_opt) in product_with_attrs {
        let entry = products_map
            .entry(product.id)
            .or_insert_with(|| ProductWithAttributes {
                product: product.clone(),
                attributes: DefaultAttributes::default(),
                stock_quantity: 0, // Initialize with zero
            });

        if let Some(attribute) = attribute_opt {
            match attribute.name.as_str() {
                "Size" => entry.attributes.size = Some(attribute.value.clone()),
                "Color" => entry.attributes.color = Some(attribute.value.clone()),
                "Weight" => entry.attributes.weight = attribute.value.parse().ok(),
                "Weight Unit" => entry.attributes.weight_unit = Some(attribute.value.clone()),
                "Width" => entry.attributes.width = attribute.value.parse().ok(),
                "Height" => entry.attributes.height = attribute.value.parse().ok(),
                _ => {}
            }
        }
    }

    // Fetch products with stock quantities
    let product_stock_quantities = products::table
        .inner_join(stock_quantities::table.on(products::id.eq(stock_quantities::product_id)))
        .select((products::id, stock_quantities::quantity))
        .load::<(i32, i32)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    // Group stock quantities by product
    for (product_id, quantity) in product_stock_quantities {
        if let Some(product_with_attrs) = products_map.get_mut(&product_id) {
            product_with_attrs.stock_quantity = quantity;
        }
    }

    // Merge products with discounts and attributes
    for (_, product_with_discount) in product_map {
        if let Some(product_with_attrs) = products_map.get_mut(&product_with_discount.product.id) {
            product_with_attrs.product = product_with_discount.product;
            // Assuming you want to merge discounts into attributes or handle them separately
        }
    }

    Ok(products_map.into_iter().map(|(_, v)| v).collect())
}
pub fn left_join_products_with_discounts(
    connection: &mut PooledConnection,
) -> Result<Vec<ProductWithDiscount>, DatabaseErrorWrapper> {
    use crate::schema::discount_products::dsl::{
        discount_id as dp_discount_id, discount_products, product_id as dp_product_id,
    };
    use crate::schema::discounts::dsl::{discounts, id as discount_id};
    use crate::schema::products::dsl::*;

    let results = products
        .left_join(discount_products.on(id.eq(dp_product_id)))
        .left_join(discounts.on(dp_discount_id.eq(discount_id)))
        .select((
            crate::schema::products::all_columns,
            crate::schema::discounts::all_columns.nullable(),
        ))
        .load::<(Product, Option<Discount>)>(connection)
        .map_err(DatabaseErrorWrapper)?;

    let mut product_map: HashMap<i32, ProductWithDiscount> = HashMap::new();

    for (product, discount) in results {
        let product_id = product.id;
        product_map
            .entry(product_id)
            .and_modify(|entry| {
                if let Some(discount) = &discount {
                    entry.discounts.push(discount.clone());
                }
            })
            .or_insert_with(|| ProductWithDiscount {
                product,
                discounts: discount.into_iter().collect(),
            });
    }

    Ok(product_map.into_iter().map(|(_, v)| v).collect())
}

pub fn fetch_product_discounts_query(
    product_id: i32,
    quantity: i32,
    connection: &mut PooledConnection,
) -> Result<Vec<Discount>, diesel::result::Error> {
    use crate::schema::{discount_products, discounts};

    discounts::table
        .inner_join(discount_products::table.on(discount_products::discount_id.eq(discounts::id)))
        .filter(discount_products::product_id.eq(product_id))
        .filter(discounts::start_date.le(diesel::dsl::now))
        .filter(discounts::end_date.ge(diesel::dsl::now))
        .filter(discounts::min_quantity.le(quantity))
        .select(discounts::all_columns)
        .load::<Discount>(connection)
}
