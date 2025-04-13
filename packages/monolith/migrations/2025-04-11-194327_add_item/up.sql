-- Your SQL goes here
CREATE TABLE marketplace_items (
    id SERIAL PRIMARY KEY,
    marketplace VARCHAR NOT NULL,
    item_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    price FLOAT NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    thumbnails TEXT[] NOT NULL,
    item_condition VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL,
    updated TIMESTAMP NOT NULL,
    seller_id VARCHAR NOT NULL,
    UNIQUE(marketplace, item_id)
);