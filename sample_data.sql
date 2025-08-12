-- Sample Data for Testing the Schema
-- Run this after migrations to populate with test data

-- Insert sample authors
INSERT INTO authors (name, date_of_birth, country_of_origin, description) VALUES 
('Gabriel García Márquez', '1927-03-06', 'Colombia', 'Nobel Prize-winning Colombian novelist, best known for One Hundred Years of Solitude'),
('J.K. Rowling', '1965-07-31', 'United Kingdom', 'British author best known for the Harry Potter fantasy series'),
('Stephen King', '1947-09-21', 'United States', 'American author of horror, supernatural fiction, suspense, and fantasy novels'),
('Isabel Allende', '1942-08-02', 'Chile', 'Chilean writer known for novels such as The House of the Spirits');

-- Insert sample books (using author IDs 1-4)
INSERT INTO books (name, summary, date_of_publication, number_of_sales, author_id) VALUES 
('Cien años de soledad', 'A multi-generational saga of the Buendía family in the fictional town of Macondo', '1967-06-05', 50000000, 1),
('Harry Potter and the Philosophers Stone', 'A young wizard discovers his magical heritage on his 11th birthday', '1997-06-26', 120000000, 2),
('The Shining', 'A family heads to an isolated hotel for the winter where a sinister presence influences the father', '1977-01-28', 700000, 3),
('La casa de los espíritus', 'The story of the del Valle and Trueba families across four generations', '1982-01-01', 20000000, 4);

-- Insert sample reviews
INSERT INTO reviews (book_id, review, score, number_of_upvotes, reviewer_name) VALUES 
(1, 'Masterpiece of magical realism. García Márquez creates a world that feels both fantastical and real.', 5, 234, 'Literature Lover'),
(1, 'Complex and beautiful, though sometimes hard to follow with so many characters.', 4, 89, 'BookWorm92'),
(2, 'Amazing start to an incredible series. Perfect for both children and adults.', 5, 1024, 'Potterhead'),
(2, 'Good story but a bit simple compared to later books in the series.', 4, 156, 'FantasyFan'),
(3, 'Terrifying and psychological. King at his best.', 5, 445, 'HorrorReader'),
(4, 'Beautiful family saga with strong female characters. Highly recommended.', 5, 267, 'BookClub2023');

-- Insert sample sales data
INSERT INTO sales (book_id, year, sales) VALUES 
(1, 2020, 2500000),
(1, 2021, 2800000),
(1, 2022, 3200000),
(1, 2023, 3100000),
(2, 2020, 8500000),
(2, 2021, 9200000),
(2, 2022, 10100000),
(2, 2023, 9800000),
(3, 2020, 150000),
(3, 2021, 180000),
(3, 2022, 200000),
(3, 2023, 175000),
(4, 2020, 1200000),
(4, 2021, 1350000),
(4, 2022, 1500000),
(4, 2023, 1400000);
