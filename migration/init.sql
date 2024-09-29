-- Step 0: Create an ENUM type for the 'role' field
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- Step 1: Create the 'users' table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    role user_role NOT NULL,
    fullname VARCHAR(255) NOT NULL,
    social_media_link VARCHAR(255),
    delete_account_date TIMESTAMP,
    update_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Categories Table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Products Table
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    stock INTEGER DEFAULT 0,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    image_url VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. Orders Table
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(50) DEFAULT 'pending',
    total_price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5. Order Items Table
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    price_at_time DECIMAL(10, 2) NOT NULL
);

-- 6. Cart Table
CREATE TABLE cart (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 7. Cart Items Table
CREATE TABLE cart_items (
    id SERIAL PRIMARY KEY,
    cart_id INTEGER REFERENCES cart(id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER DEFAULT 1
);

-- 8. Payments Table
CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE,
    payment_status VARCHAR(50) DEFAULT 'pending',
    payment_method VARCHAR(50),
    payment_date TIMESTAMPTZ DEFAULT NOW()
);

-- 9. Delivery Table
CREATE TABLE delivery (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE,
    delivery_address VARCHAR(255) NOT NULL,
    city VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,
    delivery_status VARCHAR(50) DEFAULT 'pending', -- pending, shipped, delivered, etc.
    tracking_number VARCHAR(100), -- Optional tracking number
    delivery_provider VARCHAR(100), -- E.g., UPS, FedEx, DHL
    estimated_delivery_date TIMESTAMPTZ,
    actual_delivery_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for order_id in delivery table to speed up searches
CREATE INDEX idx_delivery_order_id ON delivery(order_id);

-- 10. Reviews Table
CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 11. Discounts Table
CREATE TABLE discounts (
    id SERIAL PRIMARY KEY,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    discount_percentage DECIMAL(5, 2) CHECK (discount_percentage >= 0 AND discount_percentage <= 100),
    valid_from TIMESTAMPTZ NOT NULL,
    valid_to TIMESTAMPTZ NOT NULL
);

-- 12. Audit Logs Table (Optional for tracking changes)
CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(255),
    details TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);


-- Step 1: Generalized function to update 'update_date' on record modification for any table
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
   -- Ensure the column 'update_date' exists before updating
   IF NEW.update_date IS NOT DISTINCT FROM OLD.update_date THEN
       NEW.update_date = NOW();
   END IF;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;



-- Step 2: Trigger for 'users' table
CREATE TRIGGER update_users_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

-- Repeat for other tables
-- Example: Trigger for 'orders' table
CREATE TRIGGER update_orders_modtime
BEFORE UPDATE ON orders
FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

-- Example: Trigger for 'products' table
CREATE TRIGGER update_products_modtime
BEFORE UPDATE ON products
FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();



DO $$ 
DECLARE 
    r RECORD;
BEGIN
    FOR r IN 
        SELECT table_name
        FROM information_schema.columns 
        WHERE column_name = 'update_date'
    LOOP
        EXECUTE format('
            CREATE TRIGGER update_%I_modtime
            BEFORE UPDATE ON %I
            FOR EACH ROW
            EXECUTE PROCEDURE update_modified_column();', r.table_name, r.table_name);
    END LOOP;
END $$;
