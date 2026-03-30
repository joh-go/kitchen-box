-- Update images table to match the expected schema
-- First, drop the existing table if it has the old schema
DROP TABLE IF EXISTS images;

-- Create the images table with the correct schema
CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    original_filename TEXT,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    mime_type TEXT,
    alt TEXT,
    is_primary BOOLEAN DEFAULT false,
    position INTEGER DEFAULT 0,
    uploaded_at TIMESTAMPTZ DEFAULT now()
);
