-- Create Sales Table
-- Description: Table to track book sales by year

CREATE TABLE sales (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 1900 AND year <= 2100),
    sales BIGINT NOT NULL CHECK (sales >= 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign key constraint to books table
    CONSTRAINT fk_sales_book 
        FOREIGN KEY (book_id) 
        REFERENCES books(id) 
        ON DELETE CASCADE 
        ON UPDATE CASCADE,
    
    -- Unique constraint: one record per book per year
    CONSTRAINT unique_book_year 
        UNIQUE (book_id, year)
);

-- Indexes for better query performance
CREATE INDEX idx_sales_book_id ON sales(book_id);
CREATE INDEX idx_sales_year ON sales(year);
CREATE INDEX idx_sales_amount ON sales(sales);
