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

-- Table for criterion answers (used by both embedded and analyzed items)
CREATE TABLE criterion_answers (
    id SERIAL PRIMARY KEY,
    embedded_item_id INTEGER REFERENCES embedded_marketplace_items(id) ON DELETE CASCADE,
    analyzed_item_id INTEGER REFERENCES analyzed_marketplace_items(id) ON DELETE CASCADE,
    criterion VARCHAR NOT NULL,
    answer TEXT NOT NULL,
    CHECK (
        (embedded_item_id IS NULL AND analyzed_item_id IS NOT NULL) OR
        (embedded_item_id IS NOT NULL AND analyzed_item_id IS NULL)
    )
);