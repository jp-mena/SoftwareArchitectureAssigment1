-- Vista para estadísticas agregadas de autores
CREATE OR REPLACE VIEW author_statistics AS
SELECT 
    a.id,
    a.name as author_name,
    a.country_of_origin,
    a.date_of_birth,
    COUNT(DISTINCT b.id) as books_published,
    COALESCE(ROUND(AVG(r.score), 2), 0) as average_rating,
    COALESCE(SUM(s.sales), 0) as total_sales,
    COALESCE(COUNT(r.id), 0) as total_reviews
FROM authors a
LEFT JOIN books b ON a.id = b.author_id
LEFT JOIN reviews r ON b.id = r.book_id
LEFT JOIN sales s ON b.id = s.book_id
GROUP BY a.id, a.name, a.country_of_origin, a.date_of_birth
ORDER BY a.name;
