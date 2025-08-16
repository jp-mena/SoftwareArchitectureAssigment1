-- Fix data types in simple author stats
DROP VIEW IF EXISTS simple_author_stats;

CREATE VIEW simple_author_stats AS
SELECT 
    a.id,
    a.name as author_name,
    COUNT(b.id)::bigint as total_books,
    CASE 
        WHEN COUNT(r.score) > 0 THEN ROUND(AVG(r.score::numeric), 2)::float8
        ELSE 0::float8
    END as average_rating,
    COALESCE(SUM(s.sales)::bigint, 0::bigint) as total_sales
FROM authors a
LEFT JOIN books b ON a.id = b.author_id
LEFT JOIN reviews r ON b.id = r.book_id
LEFT JOIN sales s ON b.id = s.book_id
GROUP BY a.id, a.name
ORDER BY a.name;
