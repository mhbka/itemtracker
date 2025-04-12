-- Table for embedded marketplace items
CREATE TABLE embedded_marketplace_items (
    id SERIAL PRIMARY KEY,
    marketplace_item_id INTEGER NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    item_description TEXT NOT NULL,
    description_embedding FLOAT[] NOT NULL,
    image_embedding FLOAT[] NOT NULL,
    UNIQUE(marketplace_item_id)
);

-- Table for analyzed marketplace items
CREATE TABLE analyzed_marketplace_items (
    id SERIAL PRIMARY KEY,
    marketplace_item_id INTEGER NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    item_description TEXT NOT NULL,
    best_fit_image INTEGER NOT NULL,
    UNIQUE(marketplace_item_id)
);

-- Table for error analyzed items
CREATE TABLE error_analyzed_marketplace_items (
    id SERIAL PRIMARY KEY,
    marketplace_item_id INTEGER NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    error TEXT NOT NULL,
    UNIQUE(marketplace_item_id)
);

-- Table for error embedded items
CREATE TABLE error_embedded_marketplace_items (
    id SERIAL PRIMARY KEY,
    analyzed_item_id INTEGER NOT NULL REFERENCES analyzed_marketplace_items(id) ON DELETE CASCADE,
    error TEXT NOT NULL,
    UNIQUE(analyzed_item_id)
);