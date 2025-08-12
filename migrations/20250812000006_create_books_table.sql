-- Create Books Table
-- Description: Table to store book information with relationship to authors

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name VARCHAR(500) NOT NULL,
    summary TEXT,
    date_of_publication DATE,
    number_of_sales BIGINT DEFAULT 0 CHECK (number_of_sales >= 0),
    author_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign key constraint to authors table
    CONSTRAINT fk_books_author 
        FOREIGN KEY (author_id) 
        REFERENCES authors(id) 
        ON DELETE RESTRICT 
        ON UPDATE CASCADE
);

-- Indexes for better query performance
CREATE INDEX idx_books_name ON books(name);
CREATE INDEX idx_books_author_id ON books(author_id);
CREATE INDEX idx_books_publication_date ON books(date_of_publication);
CREATE INDEX idx_books_sales ON books(number_of_sales);
