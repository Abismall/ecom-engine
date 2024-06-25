
// src/services/stock_quantity_service.rs

pub struct Stock;

impl Stock {
    pub fn create(
        connection: &mut PgConnection,
        new_stock_quantity: crate::data::models::NewStockQuantity,
    ) -> Result<crate::data::models::StockQuantity, Error> {
        let result = diesel::insert_into(crate::data::schema::stock_quantities::table)
            .values(&new_stock_quantity)
            .returning(crate::data::schema::stock_quantities::all_columns)
            .get_result::<crate::data::models::StockQuantity>(connection);

        crate::util::map_error(result.map_err(Error::from), "Error creating stock quantity")
    }

    pub fn list(
        connection: &mut PgConnection,
    ) -> Result<Vec<crate::data::models::StockQuantity>, Error> {
        let result = crate::data::schema::stock_quantities::table
            .load::<crate::data::models::StockQuantity>(connection);

        crate::util::map_error(
            result.map_err(Error::from),
            "Error fetching stock quantities",
        )
    }

    pub fn update(
        connection: &mut PgConnection,
        id: i32,
        updated_stock_quantity: crate::data::models::StockQuantity,
    ) -> Result<crate::data::models::StockQuantity, Error> {
        let result = diesel::update(crate::data::schema::stock_quantities::table.find(id))
            .set(&updated_stock_quantity)
            .get_result::<crate::data::models::StockQuantity>(connection);

        crate::util::map_error(
            result.map_err(Error::from),
            &format!("Error updating stock quantity with ID {}", id),
        )
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> Result<(), Error> {
        let result = diesel::delete(
            crate::data::schema::stock_quantities::table
                .filter(crate::data::schema::stock_quantities::id.eq(id)),
        )
        .execute(connection)
        .map(|_| ());

        crate::util::map_error(
            result.map_err(Error::from),
            &format!("Error deleting stock quantity with ID {}", id),
        )
    }

    pub fn for_product_id(
        connection: &mut PgConnection,
        product_id: i32,
    ) -> Result<Vec<crate::data::models::StockQuantity>, Error> {
        let result = crate::data::schema::stock_quantities::table
            .filter(crate::data::schema::stock_quantities::product_id.eq(product_id))
            .select(crate::data::models::StockQuantity::as_select())
            .load::<crate::data::models::StockQuantity>(connection);

        crate::util::map_error(
            result.map_err(Error::from),
            &format!(
                "Error fetching stock quantities for product with ID {}",
                product_id
            ),
        )
    }
}


pub struct StockProvider;

impl StockProvider {
    pub fn create(
        connection: &mut PgConnection,
        new_warehouse: crate::data::models::NewWarehouse,
    ) -> Result<crate::data::models::Warehouse, Error> {
        let result = diesel::insert_into(crate::data::schema::warehouses::table)
            .values(&new_warehouse)
            .returning(crate::data::schema::warehouses::all_columns)
            .get_result::<crate::data::models::Warehouse>(connection);

        crate::util::map_error(result.map_err(Error::from), "Error creating warehouse")
    }

    pub fn list(
        connection: &mut PgConnection,
    ) -> Result<Vec<crate::data::models::Warehouse>, Error> {
        let result = crate::data::schema::warehouses::table
            .load::<crate::data::models::Warehouse>(connection);

        crate::util::map_error(result.map_err(Error::from), "Error fetching warehouses")
    }

    pub fn get(
        connection: &mut PgConnection,
        id: i32,
    ) -> Result<crate::data::models::Warehouse, Error> {
        let result = crate::data::schema::warehouses::table
            .select(crate::data::models::Warehouse::as_select())
            .filter(crate::data::schema::warehouses::id.eq(id))
            .first::<crate::data::models::Warehouse>(connection);

        crate::util::map_error(
            result.map_err(Error::from),
            &format!("Error fetching warehouse with ID {}", id),
        )
    }

    pub fn get_stock(
        connection: &mut PgConnection,
        id: i32,
    ) -> Result<Vec<crate::data::models::StockQuantity>, Error> {
        let result = crate::data::schema::stock_quantities::table
            .filter(crate::data::schema::stock_quantities::warehouse_id.eq(id))
            .select(crate::data::models::StockQuantity::as_select())
            .load::<crate::data::models::StockQuantity>(connection);

        crate::util::map_error(
            result.map_err(Error::from),
            &format!(
                "Error fetching stock quantities for warehouse with ID {}",
                id
            ),
        )
    }
}
