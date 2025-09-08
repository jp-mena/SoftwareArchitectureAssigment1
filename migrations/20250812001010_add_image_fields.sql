-- Add image fields to authors and books tables
-- Description: Add support for author photos and book cover images

-- Add image_path column to authors table
ALTER TABLE authors ADD COLUMN image_path VARCHAR(500);

-- Add cover_image_path column to books table  
ALTER TABLE books ADD COLUMN cover_image_path VARCHAR(500);

-- Add comments for documentation
COMMENT ON COLUMN authors.image_path IS 'Path to author photo image file';
COMMENT ON COLUMN books.cover_image_path IS 'Path to book cover image file';
