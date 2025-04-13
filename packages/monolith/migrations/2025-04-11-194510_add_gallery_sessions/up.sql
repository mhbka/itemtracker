-- Table for gallery sessions
CREATE TABLE gallery_sessions (
    id SERIAL PRIMARY KEY,
    gallery_id UUID NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    used_evaluation_criteria JSONB NOT NULL
);

-- Table for embedded marketplace items
CREATE TABLE embedded_marketplace_items (
    id SERIAL PRIMARY KEY,
    gallery_session_id INTEGER NOT NULL REFERENCES gallery_sessions(id) ON DELETE CASCADE,
    marketplace_item_id INTEGER NOT NULL REFERENCES marketplace_items(id) ON DELETE CASCADE,
    item_description TEXT NOT NULL,
    description_embedding FLOAT4[] NOT NULL,
    image_embedding FLOAT4[] NOT NULL,
    evaluation_answers JSONB[] NOT NULL,
    UNIQUE(marketplace_item_id, gallery_session_id)
);
