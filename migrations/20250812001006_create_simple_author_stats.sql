-- Create simple author statistics view
CREATE VIEW simple_author_stats AS
SELECT 
    a.id,
    a.name as author_name,
    COUNT(b.id) as total_books
FROM authors a
LEFT JOIN books b ON a.id = b.author_id
GROUP BY a.id, a.name
ORDER BY a.name;
