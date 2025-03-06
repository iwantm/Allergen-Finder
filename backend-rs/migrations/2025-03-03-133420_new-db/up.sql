-- Your SQL goes here
CREATE TABLE IF NOT EXISTS products(
    barcode TEXT PRIMARY KEY,
    product_name TEXT,
    ingredients TEXT[] NOT NULL,
    allergens TEXT[],
    traces TEXT[],
    user_id TEXT
)