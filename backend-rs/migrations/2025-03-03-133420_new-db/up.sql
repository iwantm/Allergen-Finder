-- Your SQL goes here
CREATE TABLE products (
    barcode TEXT PRIMARY KEY,
    product_name TEXT,
    ingredients TEXT[] NOT NULL,
    allergens TEXT[],
    traces TEXT[]
)