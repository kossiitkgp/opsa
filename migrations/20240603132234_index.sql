-- Enable the necessary extensions
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Create the tsvector column and its GIN index for full-text search
ALTER TABLE messages ADD COLUMN IF NOT EXISTS msg_tsv tsvector;
UPDATE messages SET msg_tsv = to_tsvector('english', msg_text) WHERE msg_tsv IS NULL;
CREATE INDEX IF NOT EXISTS msg_tsv_idx ON messages USING GIN(msg_tsv);
-- Optional: Add a trigger to keep msg_tsv updated automatically

-- Create the trigram GIN index for fuzzy/similarity search
CREATE INDEX IF NOT EXISTS trgm_idx ON messages USING GIN (msg_text gin_trgm_ops);