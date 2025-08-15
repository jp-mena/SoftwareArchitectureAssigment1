-- Fix column name in author statistics view
DROP VIEW IF EXISTS author_statistics;

CREATE VIEW author_statistics AS
SELECT 
    a.id,
    a.name as author_name,
    a.country_of_origin as country,
    COUNT(DISTINCT b.id)::bigint as books_published,
    CASE 
        WHEN COUNT(r.score) > 0 THEN ROUND(AVG(r.score)::numeric, 2)::float8
        ELSE 0::float8
    END as average_rating,
    COALESCE(SUM(s.sales)::bigint, 0::bigint) as total_sales,
    COUNT(r.id)::bigint as total_reviews
FROM authors a
LEFT JOIN books b ON a.id = b.author_id
LEFT JOIN reviews r ON b.id = r.book_id
LEFT JOIN sales s ON b.id = s.book_id
GROUP BY a.id, a.name, a.country_of_origin
ORDER BY a.name;
