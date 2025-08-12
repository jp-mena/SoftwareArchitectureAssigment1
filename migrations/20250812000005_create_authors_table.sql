-- Create Authors Table
-- Description: Table to store author information with personal details

CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    date_of_birth DATE,
    country_of_origin VARCHAR(100),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster name searches
CREATE INDEX idx_authors_name ON authors(name);

-- Index for country searches  
CREATE INDEX idx_authors_country ON authors(country_of_origin);
