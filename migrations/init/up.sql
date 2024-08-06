-- Categories table
CREATE TABLE "categories" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR NOT NULL
);

-- Warehouses table
CREATE TABLE "warehouses" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

-- Brands table
CREATE TABLE "brands" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "description" VARCHAR
);

-- Discounts table
CREATE TABLE "discounts" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "discount_type" VARCHAR NOT NULL,
    "value" INT4 NOT NULL,
    "start_date" TIMESTAMP DEFAULT NOW() NOT NULL,
    "end_date" TIMESTAMP DEFAULT NOW() NOT NULL,
    "min_quantity" INT4 DEFAULT 1
);

-- Products table
CREATE TABLE "products" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "in_stock" BOOL DEFAULT false NOT NULL,
    "category_id" INT4,
    "brand_id" INT4,
    "price" INT4 NOT NULL,
    "tax_rate" INT4 NOT NULL,
    FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE SET NULL,
    FOREIGN KEY ("brand_id") REFERENCES "brands"("id") ON DELETE SET NULL
);

-- Stock quantities table
CREATE TABLE "stock_quantities" (
    "product_id" INT4 NOT NULL,
    "warehouse_id" INT4 NOT NULL DEFAULT 1,
    "quantity" INT4 NOT NULL,
    PRIMARY KEY ("product_id", "warehouse_id"),
    FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE CASCADE,
    FOREIGN KEY ("warehouse_id") REFERENCES "warehouses"("id") ON DELETE CASCADE
);
-- Ensure there's a warehouse with ID 1
INSERT INTO warehouses (name) VALUES
('Default_1'),
('Default_3'),
('Default_3');


-- Carts table
CREATE TABLE "carts" (
    "id" SERIAL PRIMARY KEY,
    "is_active" BOOL DEFAULT false
);

-- Discount brands table
CREATE TABLE "discount_brands" (
    "discount_id" INT4 REFERENCES discounts(id),
    "brand_id" INT4 REFERENCES brands(id),
    PRIMARY KEY ("discount_id", "brand_id"),
    FOREIGN KEY (discount_id) REFERENCES discounts(id),
    FOREIGN KEY (brand_id) REFERENCES brands(id)
);

-- Discount categories table
CREATE TABLE "discount_categories" (
    "discount_id" INT4 REFERENCES discounts(id),
    "category_id" INT4 REFERENCES categories(id),
    PRIMARY KEY ("discount_id", "category_id"),
    FOREIGN KEY (discount_id) REFERENCES discounts(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- Discount products table
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
    "warehouse_id" INT4 NOT NULL,
    "quantity" INT4 NOT NULL,
    CONSTRAINT unique_cart_product UNIQUE ("cart_id", "product_id"),
    FOREIGN KEY ("cart_id") REFERENCES "carts"("id") ON DELETE CASCADE,
    FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE SET NULL,
    FOREIGN KEY ("warehouse_id") REFERENCES "warehouses"("id") ON DELETE CASCADE
);
-- Attributes table
CREATE TABLE "attributes" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "value" VARCHAR NOT NULL
);

-- Product attributes junction table
CREATE TABLE "product_attributes" (
    "product_id" INT4 NOT NULL,
    "attribute_id" INT4 NOT NULL,
    PRIMARY KEY ("product_id", "attribute_id"),
    FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE CASCADE,
    FOREIGN KEY ("attribute_id") REFERENCES "attributes"("id") ON DELETE CASCADE
);

-- Function to decrease stock quantity on INSERT
CREATE OR REPLACE FUNCTION decrease_stock_quantity()
RETURNS TRIGGER AS $$
BEGIN
    RAISE NOTICE 'Decreasing stock for product_id: %, warehouse_id: %, quantity: %', NEW.product_id, NEW.warehouse_id, NEW.quantity;
    
    UPDATE stock_quantities
    SET quantity = quantity - NEW.quantity
    WHERE product_id = NEW.product_id
      AND warehouse_id = NEW.warehouse_id;

    RAISE NOTICE 'Stock updated: product_id: %, warehouse_id: %, new quantity: %', NEW.product_id, NEW.warehouse_id, NEW.quantity;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to increase stock quantity on DELETE
CREATE OR REPLACE FUNCTION increase_stock_quantity()
RETURNS TRIGGER AS $$
BEGIN
    RAISE NOTICE 'Increasing stock for product_id: %, warehouse_id: %, quantity: %', OLD.product_id, OLD.warehouse_id, OLD.quantity;

    UPDATE stock_quantities
    SET quantity = quantity + OLD.quantity
    WHERE product_id = OLD.product_id
      AND warehouse_id = OLD.warehouse_id;

    RAISE NOTICE 'Stock updated: product_id: %, warehouse_id: %, new quantity: %', OLD.product_id, OLD.warehouse_id, OLD.quantity;

    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_stock_quantity()
RETURNS TRIGGER AS $$
BEGIN
    -- Logging the initial state
    RAISE NOTICE 'Updating stock for product_id: %, warehouse_id: %, old quantity: %, new quantity: %', OLD.product_id, OLD.warehouse_id, OLD.quantity, NEW.quantity;

    -- Increase the stock by the old quantity
    UPDATE stock_quantities
    SET quantity = quantity + OLD.quantity
    WHERE product_id = OLD.product_id
      AND warehouse_id = OLD.warehouse_id;

    -- Logging after increasing the old quantity
    RAISE NOTICE 'Stock after increasing old quantity: product_id: %, warehouse_id: %, quantity: %', OLD.product_id, OLD.warehouse_id, (SELECT quantity FROM stock_quantities WHERE product_id = OLD.product_id AND warehouse_id = OLD.warehouse_id);

    -- Decrease the stock by the new quantity
    UPDATE stock_quantities
    SET quantity = quantity - NEW.quantity
    WHERE product_id = NEW.product_id
      AND warehouse_id = NEW.warehouse_id;

    -- Logging after decreasing the new quantity
    RAISE NOTICE 'Stock after decreasing new quantity: product_id: %, warehouse_id: %, quantity: %', NEW.product_id, NEW.warehouse_id, (SELECT quantity FROM stock_quantities WHERE product_id = NEW.product_id AND warehouse_id = NEW.warehouse_id);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Recreate the trigger for INSERT
CREATE TRIGGER after_order_line_insert
AFTER INSERT ON order_lines
FOR EACH ROW
EXECUTE FUNCTION decrease_stock_quantity();

-- Recreate the trigger for DELETE
CREATE TRIGGER after_order_line_delete
AFTER DELETE ON order_lines
FOR EACH ROW
EXECUTE FUNCTION increase_stock_quantity();

-- Recreate the trigger for UPDATE
CREATE TRIGGER after_order_line_update
AFTER UPDATE ON order_lines
FOR EACH ROW
EXECUTE FUNCTION update_stock_quantity();

-- Insert categories

-- Insert categories
INSERT INTO categories (name, description) VALUES
('Electronics', 'Electronic items and gadgets'),
('Clothing', 'Apparel and garments'),
('Books', 'Books and literature');

-- Insert brands
INSERT INTO brands (name, description) VALUES
('Brand A', 'Description for Brand A'),
('Brand B', 'Description for Brand B'),
('Brand C', 'Description for Brand C');


-- Insert discounts
INSERT INTO discounts (name, discount_type, value, start_date, end_date, min_quantity) VALUES
('Discount 1', 'Percentage', 10, NOW(), NOW() + INTERVAL '30 days', 1),
('Discount 2', 'Fixed', 50, NOW(), NOW() + INTERVAL '30 days', 1),
('Discount 3', 'Percentage', 20, NOW(), NOW() + INTERVAL '30 days', 1);

-- Insert products
DO $$
BEGIN
    FOR i IN 1..1 LOOP
        INSERT INTO products (name, in_stock, category_id, brand_id, price, tax_rate) VALUES
        (format('Product %s', i), true, (i % 3) + 1, (i % 3) + 1, (i * 10) % 1000, 5);
    END LOOP;
END $$;

-- Insert stock quantities
DO $$
BEGIN
    FOR i IN 1..1 LOOP
        INSERT INTO stock_quantities (product_id, warehouse_id, quantity) VALUES
        (i, (i % 3) + 1, (i * 2) % 100);
    END LOOP;
END $$;

-- Insert attributes
INSERT INTO attributes (name, value) VALUES
('Size', 'Large'),
('Color', 'Red'),
('Weight', '500g'),
('Weight Unit', 'grams'),
('Width', '10cm'),
('Height', '20cm');

-- Insert product attributes
DO $$
BEGIN
    FOR i IN 1..1 LOOP
        INSERT INTO product_attributes (product_id, attribute_id) VALUES
        (i, 1), -- Size
        (i, 2), -- Color
        (i, 3), -- Weight
        (i, 4), -- Weight Unit
        (i, 5), -- Width
        (i, 6); -- Height
    END LOOP;
END $$;

-- Insert discount brands
INSERT INTO discount_brands (discount_id, brand_id) VALUES
(1, 1);


-- Insert discount categories
INSERT INTO discount_categories (discount_id, category_id) VALUES
(1, 1);

-- Insert discount products
DO $$
BEGIN
    FOR i IN 1..1 LOOP
        INSERT INTO discount_products (discount_id, product_id) VALUES
        ((i % 3) + 1, i);
    END LOOP;
END $$;

-- Insert carts
INSERT INTO carts (is_active) VALUES
(true),
(false);

-- Insert order lines
-- DO $$
-- BEGIN
--     FOR i IN 1..10000 LOOP
--         INSERT INTO order_lines (cart_id, product_id, warehouse_id, quantity) VALUES
--         ((i % 2) + 1, i, (i % 3) + 1, (i * 3) % 50);
--     END LOOP;
-- END $$;

-- -- Initial data insertion

-- -- Insert 3 categories
-- DO $$
-- BEGIN
--     FOR i IN 1..3 LOOP
--         INSERT INTO categories (name, description) 
--         VALUES ('Category ' || i, 'Description for Category ' || i);
--     END LOOP;
-- END $$;

-- -- Insert 3 warehouses
-- DO $$
-- BEGIN
--     FOR i IN 1..3 LOOP
--         INSERT INTO warehouses (name) 
--         VALUES ('Warehouse ' || i);
--     END LOOP;
-- END $$;

-- -- Insert 3 brands
-- DO $$
-- BEGIN
--     FOR i IN 1..3 LOOP
--         INSERT INTO brands (name, description) 
--         VALUES ('Brand ' || i, 'Description for Brand ' || i);
--     END LOOP;
-- END $$;

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
