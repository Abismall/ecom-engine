diesel::table! {
    brands (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    carts (id) {
        id -> Int4,
        is_active -> Bool,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    discount_brands (discount_id, brand_id) {
        discount_id -> Int4,
        brand_id -> Int4,
    }
}

diesel::table! {
    discount_categories (discount_id, category_id) {
        discount_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    discount_products (discount_id, product_id) {
        discount_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    discounts (id) {
        id -> Int4,
        name -> Varchar,
        discount_type -> Varchar,
        value -> Int4,
        start_date -> Timestamp,
        end_date -> Timestamp,
        min_quantity -> Int4,
    }
}

diesel::table! {
    order_lines (id) {
        id -> Int4,
        cart_id -> Int4,
        product_id -> Int4,
        warehouse_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        in_stock -> Bool,
        category_id -> Nullable<Int4>,
        brand_id -> Nullable<Int4>,
        price -> Int4,
        tax_rate -> Int4,
    }
}

diesel::table! {
    attributes (id) {
        id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    product_attributes (product_id, attribute_id) {
        product_id -> Int4,
        attribute_id -> Int4,
    }
}

diesel::table! {
    stock_quantities (product_id, warehouse_id) {
        product_id -> Int4,
        warehouse_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    warehouses (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(discount_brands -> brands (brand_id));
diesel::joinable!(discount_brands -> discounts (discount_id));
diesel::joinable!(discount_categories -> categories (category_id));
diesel::joinable!(discount_categories -> discounts (discount_id));
diesel::joinable!(discount_products -> discounts (discount_id));
diesel::joinable!(discount_products -> products (product_id));
diesel::joinable!(order_lines -> carts (cart_id));
diesel::joinable!(order_lines -> products (product_id));
diesel::joinable!(order_lines -> warehouses (warehouse_id));
diesel::joinable!(products -> brands (brand_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(product_attributes -> products (product_id));
diesel::joinable!(product_attributes -> attributes (attribute_id));
diesel::joinable!(stock_quantities -> products (product_id));
diesel::joinable!(stock_quantities -> warehouses (warehouse_id));

diesel::allow_tables_to_appear_in_same_query!(
    attributes,
    brands,
    carts,
    categories,
    discounts,
    discount_brands,
    discount_categories,
    discount_products,
    order_lines,
    products,
    product_attributes,
    stock_quantities,
    warehouses,
);
