-- Modify Reviews Table  
-- Description: Transform the existing reviews table to the expected book reviews structure

-- First, drop the old columns that we don't need
ALTER TABLE reviews DROP COLUMN IF EXISTS title;
ALTER TABLE reviews DROP COLUMN IF EXISTS content;

-- Add the new columns we need
ALTER TABLE reviews 
    ADD COLUMN IF NOT EXISTS book_id INTEGER,
    ADD COLUMN IF NOT EXISTS review TEXT,
    ADD COLUMN IF NOT EXISTS score INTEGER,
    ADD COLUMN IF NOT EXISTS number_of_upvotes INTEGER DEFAULT 0,
    ADD COLUMN IF NOT EXISTS reviewer_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;

-- Update the created_at column to have timezone
ALTER TABLE reviews ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE reviews ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;

-- Make required columns NOT NULL (after adding them)
UPDATE reviews SET book_id = 1 WHERE book_id IS NULL; -- Temporary default
UPDATE reviews SET review = 'Migrated review' WHERE review IS NULL; -- Temporary default
UPDATE reviews SET score = 5 WHERE score IS NULL; -- Temporary default

ALTER TABLE reviews ALTER COLUMN book_id SET NOT NULL;
ALTER TABLE reviews ALTER COLUMN review SET NOT NULL;
ALTER TABLE reviews ALTER COLUMN score SET NOT NULL;

-- Add constraints (without IF NOT EXISTS)
DO $$ 
BEGIN
    -- Add score constraint if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'chk_reviews_score') THEN
        ALTER TABLE reviews ADD CONSTRAINT chk_reviews_score CHECK (score >= 1 AND score <= 5);
    END IF;
    
    -- Add upvotes constraint if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'chk_reviews_upvotes') THEN
        ALTER TABLE reviews ADD CONSTRAINT chk_reviews_upvotes CHECK (number_of_upvotes >= 0);
    END IF;
    
    -- Add foreign key constraint if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_reviews_book') THEN
        ALTER TABLE reviews ADD CONSTRAINT fk_reviews_book 
            FOREIGN KEY (book_id) 
            REFERENCES books(id) 
            ON DELETE CASCADE 
            ON UPDATE CASCADE;
    END IF;
END $$;

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_reviews_book_id ON reviews(book_id);
CREATE INDEX IF NOT EXISTS idx_reviews_score ON reviews(score);
CREATE INDEX IF NOT EXISTS idx_reviews_upvotes ON reviews(number_of_upvotes);
CREATE INDEX IF NOT EXISTS idx_reviews_created_at ON reviews(created_at);
