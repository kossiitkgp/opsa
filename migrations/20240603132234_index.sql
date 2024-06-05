-- Add migration script here
ALTER TABLE messages
    ADD COLUMN textsearchable_index_col tsvector
               GENERATED ALWAYS AS (to_tsvector('english', msg_text)) STORED;

CREATE INDEX textsearch_idx ON messages USING GIN (textsearchable_index_col);