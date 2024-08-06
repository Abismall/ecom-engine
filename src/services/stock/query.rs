use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    error::DatabaseErrorWrapper,
    services::order::model::{NewWarehouse, Warehouse},
    {
        postgres::PooledConnection,
        schema::{stock_quantities, warehouses},
    },
};

use super::model::{NewStockQuantity, StockQuantity};

pub fn insert_stock_quantity_query(
    connection: &mut PooledConnection,
    new_stock_quantity: NewStockQuantity,
) -> Result<usize, DatabaseErrorWrapper> {
    use crate::schema::stock_quantities::dsl::*;

    let stock_entry = NewStockQuantity {
        warehouse_id: new_stock_quantity.warehouse_id.or(Some(1)), // Default to warehouse_id 1 if None
        ..new_stock_quantity
    };

    diesel::insert_into(stock_quantities)
        .values(&stock_entry)
        .on_conflict((warehouse_id, product_id))
        .do_update()
        .set(quantity.eq(quantity + new_stock_quantity.quantity))
        .execute(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn load_stock_quantity_query(
    connection: &mut PooledConnection,
) -> Result<Vec<StockQuantity>, DatabaseErrorWrapper> {
    crate::schema::stock_quantities::table
        .load::<StockQuantity>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn set_stock_quantity_for_product(
    connection: &mut PooledConnection,
    updated_stock_quantity: StockQuantity,
) -> Result<StockQuantity, DatabaseErrorWrapper> {
    use crate::schema::stock_quantities::dsl::*;

    diesel::update(
        stock_quantities
            .filter(product_id.eq(updated_stock_quantity.product_id))
            .filter(warehouse_id.eq(updated_stock_quantity.warehouse_id)),
    )
    .set(&StockQuantity {
        warehouse_id: updated_stock_quantity.warehouse_id,
        product_id: updated_stock_quantity.product_id,
        quantity: updated_stock_quantity.quantity,
    })
    .get_result::<StockQuantity>(connection)
    .map_err(DatabaseErrorWrapper)
}

pub fn delete_stock_quantity_from_product_query(
    connection: &mut PooledConnection,
    product_id: i32,
) -> Result<usize, DatabaseErrorWrapper> {
    diesel::delete(
        stock_quantities::table
            .filter(crate::schema::stock_quantities::columns::product_id.eq(product_id)),
    )
    .execute(connection)
    .map_err(DatabaseErrorWrapper)
}

pub fn select_stock_quantity_for_product(
    product_id: i32,
    connection: &mut PooledConnection,
) -> Result<Vec<StockQuantity>, DatabaseErrorWrapper> {
    crate::schema::stock_quantities::table
        .filter(crate::schema::stock_quantities::product_id.eq(product_id))
        .select(StockQuantity::as_select())
        .load::<StockQuantity>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn insert_warehouse_query(
    connection: &mut PooledConnection,
    new_warehouse: NewWarehouse,
) -> Result<Warehouse, DatabaseErrorWrapper> {
    diesel::insert_into(crate::schema::warehouses::table)
        .values(&new_warehouse)
        .returning(crate::schema::warehouses::all_columns)
        .get_result::<Warehouse>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn load_warehouses_query(
    connection: &mut PooledConnection,
) -> Result<Vec<Warehouse>, DatabaseErrorWrapper> {
    crate::schema::warehouses::table
        .load::<Warehouse>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn select_warehouse_query(
    connection: &mut PooledConnection,
    ware_id: i32,
) -> Result<Warehouse, DatabaseErrorWrapper> {
    crate::schema::warehouses::table
        .select(Warehouse::as_select())
        .filter(crate::schema::warehouses::id.eq(ware_id))
        .first::<Warehouse>(connection)
        .map_err(DatabaseErrorWrapper)
}

pub fn select_stock_quantity_for_warehouse_query(
    connection: &mut PooledConnection,
    ware_id: i32,
) -> Result<Vec<StockQuantity>, DatabaseErrorWrapper> {
    crate::schema::stock_quantities::table
        .filter(crate::schema::stock_quantities::warehouse_id.eq(ware_id))
        .select(StockQuantity::as_select())
        .load::<StockQuantity>(connection)
        .map_err(DatabaseErrorWrapper)
}
pub fn delete_warehouse_query(
    connection: &mut PooledConnection,
    ware_id: i32,
) -> Result<usize, DatabaseErrorWrapper> {
    diesel::delete(warehouses::table.filter(crate::schema::warehouses::columns::id.eq(ware_id)))
        .execute(connection)
        .map_err(DatabaseErrorWrapper)
}
