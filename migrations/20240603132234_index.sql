-- Create the pg_trgm extension if it doesn't already exist
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create a GIN index on the 'msg_text' column for efficient similarity searches
-- The 'gin_trgm_ops' operator class is specifically for trigram-based searches
CREATE INDEX trgm_idx ON messages USING GIN (msg_text gin_trgm_ops);

-- The previous script for tsvector is kept as it's a good practice for full-text search,
-- which can be used in conjunction with trigram similarity searches.
ALTER TABLE messages
    ADD COLUMN textsearchable_index_col tsvector
        GENERATED ALWAYS AS (to_tsvector('english', msg_text)) STORED;

CREATE INDEX textsearch_idx ON messages USING GIN (textsearchable_index_col);