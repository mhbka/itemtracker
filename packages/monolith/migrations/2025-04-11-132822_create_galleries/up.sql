-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE galleries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    scraping_periodicity TEXT NOT NULL,
    search_criteria JSONB NOT NULL,
    evaluation_criteria JSONB NOT NULL,
    mercari_last_scraped_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);