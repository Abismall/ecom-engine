CREATE TABLE "categories" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL
);

CREATE TABLE "warehouses" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

CREATE TABLE "brands" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR
);

CREATE TABLE "discounts" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "discount_type" VARCHAR NOT NULL,
    "value" INT4 NOT NULL,
    "start_date" TIMESTAMP DEFAULT NOW() NOT NULL,
    "end_date" TIMESTAMP DEFAULT NOW() NOT NULL,
    "min_quantity" INT4 DEFAULT 1
);

CREATE TABLE "products" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "in_stock" BOOL DEFAULT false NOT NULL,
    "size" VARCHAR,
    "color" VARCHAR,
    "weight" INT4 NOT NULL,
    "weight_unit" VARCHAR,
    "width" INT4 NOT NULL,
    "height" INT4 NOT NULL,
    "category_id" INT4,
    "brand_id" INT4,
    "price" INT4 NOT NULL,
    "tax_rate" INT4 NOT NULL,
    FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE SET NULL,
    FOREIGN KEY ("brand_id") REFERENCES "brands"("id") ON DELETE SET NULL
);

CREATE TABLE "stock_quantities" (
    "id" SERIAL PRIMARY KEY,
    "product_id" INT4 NOT NULL,
    "warehouse_id" INT4 NOT NULL,
    "quantity" INT4 NOT NULL,
    FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE CASCADE,
    FOREIGN KEY ("warehouse_id") REFERENCES "warehouses"("id") ON DELETE CASCADE
);

CREATE TABLE "carts" (
    "id" SERIAL PRIMARY KEY,
    "is_active" BOOL DEFAULT false
);

CREATE TABLE "discount_brands" (
    "discount_id" INT4 REFERENCES discounts(id),
    "brand_id" INT4 REFERENCES brands(id),
    PRIMARY KEY ("discount_id", "brand_id"),
    FOREIGN KEY (discount_id) REFERENCES discounts(id),
    FOREIGN KEY (brand_id) REFERENCES brands(id)
);

CREATE TABLE "discount_categories" (
    "discount_id" INT4 REFERENCES discounts(id),
    "category_id" INT4 REFERENCES categories(id),
    PRIMARY KEY ("discount_id", "category_id"),
    FOREIGN KEY (discount_id) REFERENCES discounts(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE "discount_products" (
    "discount_id" INT4 REFERENCES discounts(id),
    "product_id" INT4 REFERENCES products(id),
    PRIMARY KEY ("discount_id", "product_id"),
    FOREIGN KEY (discount_id) REFERENCES discounts(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE TABLE "order_lines" (
    "id" SERIAL PRIMARY KEY,
    "cart_id" INT4 NOT NULL,
    "product_id" INT4 NOT NULL,
    "quantity" INT4 NOT NULL,
    FOREIGN KEY ("cart_id") REFERENCES "carts"("id") ON DELETE CASCADE,
    FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE SET NULL
);

-- Initial data insertion

-- Insert 3 categories
DO $$
BEGIN
    FOR i IN 1..3 LOOP
        INSERT INTO categories (name, description) 
        VALUES ('Category ' || i, 'Description for Category ' || i);
    END LOOP;
END $$;

-- Insert 3 warehouses
DO $$
BEGIN
    FOR i IN 1..3 LOOP
        INSERT INTO warehouses (name) 
        VALUES ('Warehouse ' || i);
    END LOOP;
END $$;

-- Insert 3 brands
DO $$
BEGIN
    FOR i IN 1..3 LOOP
        INSERT INTO brands (name, description) 
        VALUES ('Brand ' || i, 'Description for Brand ' || i);
    END LOOP;
END $$;

-- -- Insert 10 discounts with varying active statuses and values
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO discounts (name, discount_type, value, start_date, end_date, min_quantity)
--         VALUES 
--         (
--             'Discount ' || i, 
--             CASE 
--                 WHEN i % 2 = 0 THEN 'percentage' 
--                 ELSE 'fixed' 
--             END, 
--             5 * i, 
--             CASE 
--                 WHEN i % 2 = 0 THEN NOW() - (i || ' days')::interval 
--                 ELSE NOW() + ((i * 2) || ' days')::interval 
--             END, 
--             CASE 
--                 WHEN i % 2 = 0 THEN NOW() + ((i * 2) || ' days')::interval 
--                 ELSE NOW() + ((i * 4) || ' days')::interval 
--             END, 
--             1 + i
--         );
--     END LOOP;
-- END $$;

-- Insert 10000 products, each belonging to one of the 3 categories and brands
-- DO $$
-- BEGIN
--     FOR i IN 1..1500 LOOP
--         INSERT INTO products (
--             name, in_stock, size, color, weight, weight_unit, width, height, category_id, brand_id, price, tax_rate
--         ) VALUES (
--             'Product ' || i, true, 'M', 'Color ' || i, 500 + i, 'grams', 10 + i, 20 + i, ((i - 1) % 3) + 1, ((i - 1) % 3) + 1, 1000 + (i * 10), 10 + i
--         );
--     END LOOP;
-- END $$;

-- Insert 10000 stock quantities
-- DO $$
-- BEGIN
--     FOR i IN 1..100 LOOP
--         INSERT INTO stock_quantities (product_id, warehouse_id, quantity) 
--         VALUES (i, ((i - 1) % 3) + 1, 100 + i);
--     END LOOP;
-- END $$;

-- Insert 10 carts
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO carts (is_active) 
--         VALUES (i % 2 = 0);
--     END LOOP;
-- END $$;

-- Insert discount_brand associations ensuring no duplicates
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO discount_brands (discount_id, brand_id) 
--         VALUES (i, ((i - 1) % 3) + 1);
--     END LOOP;
-- END $$;

-- Insert discount_category associations ensuring no duplicates
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO discount_categories (discount_id, category_id) 
--         VALUES (i, ((i - 1) % 3) + 1);
--     END LOOP;
-- END $$;

-- Insert discount_product associations ensuring no duplicates
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO discount_products (discount_id, product_id) 
--         VALUES (i, ((i - 1) % 3) + 1);
--     END LOOP;
-- END $$;

-- Insert additional discounts ensuring no duplicates and varying count
-- DO $$
-- DECLARE
--     discount_count INT := 0;
-- BEGIN
--     FOR i IN 1..10 LOOP
--         discount_count := (RANDOM() * 5)::INT;
--         FOR j IN 1..discount_count LOOP
--             INSERT INTO discount_brands (discount_id, brand_id) 
--             VALUES ((1 + (RANDOM() * 9)::INT), ((i - 1) % 3) + 1)
--             ON CONFLICT DO NOTHING;

--             INSERT INTO discount_categories (discount_id, category_id) 
--             VALUES ((1 + (RANDOM() * 9)::INT), ((i - 1) % 3) + 1)
--             ON CONFLICT DO NOTHING;

--             INSERT INTO discount_products (discount_id, product_id) 
--             VALUES ((1 + (RANDOM() * 9)::INT), ((i - 1) % 3) + 1)
--             ON CONFLICT DO NOTHING;
--         END LOOP;
--     END LOOP;
-- END $$;

-- Insert 10 order lines
-- DO $$
-- BEGIN
--     FOR i IN 1..10 LOOP
--         INSERT INTO order_lines (cart_id, product_id, quantity) 
--         VALUES (i, i, 1 + i);
--     END LOOP;
-- END $$;
