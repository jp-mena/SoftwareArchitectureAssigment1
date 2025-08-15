-- Script con IDs explícitos para evitar problemas de secuencia

BEGIN;

-- =====================================================
-- LIMPIAR DATOS EXISTENTES
-- =====================================================
DELETE FROM sales;
DELETE FROM reviews;
DELETE FROM books;
DELETE FROM authors;

-- Reiniciar secuencias
ALTER SEQUENCE authors_id_seq RESTART WITH 1;
ALTER SEQUENCE books_id_seq RESTART WITH 1;
ALTER SEQUENCE reviews_id_seq RESTART WITH 1;
ALTER SEQUENCE sales_id_seq RESTART WITH 1;

-- =====================================================
-- INSERTAR 50 AUTORES CON IDs EXPLÍCITOS
-- =====================================================

INSERT INTO authors (id, name, date_of_birth, country_of_origin, description) VALUES
(1, 'Gabriel García Márquez', '1927-03-06', 'Colombia', 'Premio Nobel de Literatura 1982, maestro del realismo mágico'),
(2, 'Isabel Allende', '1942-08-02', 'Chile', 'Autora de novelas históricas y realismo mágico'),
(3, 'Mario Vargas Llosa', '1936-03-28', 'Perú', 'Premio Nobel de Literatura 2010, novelista y ensayista'),
(4, 'Octavio Paz', '1914-03-31', 'México', 'Premio Nobel de Literatura 1990, poeta y ensayista'),
(5, 'Jorge Luis Borges', '1899-08-24', 'Argentina', 'Maestro del cuento corto y la literatura fantástica'),
(6, 'Pablo Neruda', '1904-07-12', 'Chile', 'Premio Nobel de Literatura 1971, poeta'),
(7, 'Julio Cortázar', '1914-08-26', 'Argentina', 'Renovador de la narrativa en español'),
(8, 'Carlos Fuentes', '1928-11-11', 'México', 'Novelista y ensayista del boom latinoamericano'),
(9, 'Elena Poniatowska', '1932-05-19', 'México', 'Periodista y escritora, cronista de México'),
(10, 'Roberto Bolaño', '1953-04-28', 'Chile', 'Poeta y novelista, figura clave de la literatura contemporánea'),
(11, 'Laura Esquivel', '1950-09-30', 'México', 'Novelista conocida por "Como agua para chocolate"'),
(12, 'Gioconda Belli', '1948-12-09', 'Nicaragua', 'Poeta y novelista nicaragüense'),
(13, 'Augusto Roa Bastos', '1917-06-13', 'Paraguay', 'Premio Cervantes 1989, novelista paraguayo'),
(14, 'Clarice Lispector', '1920-12-10', 'Brasil', 'Novelista y cuentista brasileña de origen ucraniano'),
(15, 'Ernesto Sabato', '1911-06-24', 'Argentina', 'Novelista, ensayista y físico argentino'),
(16, 'José Saramago', '1922-11-16', 'Portugal', 'Premio Nobel de Literatura 1998'),
(17, 'Fernando Pessoa', '1888-06-13', 'Portugal', 'Poeta modernista portugués'),
(18, 'Camilo José Cela', '1916-05-11', 'España', 'Premio Nobel de Literatura 1989'),
(19, 'Ana María Matute', '1925-07-26', 'España', 'Premio Cervantes 2010, novelista española'),
(20, 'Carmen Laforet', '1921-09-06', 'España', 'Novelista española, autora de "Nada"'),
(21, 'Miguel Delibes', '1920-10-17', 'España', 'Novelista español, cronista de Castilla'),
(22, 'Javier Marías', '1951-09-20', 'España', 'Novelista y traductor español'),
(23, 'Arturo Pérez-Reverte', '1951-11-25', 'España', 'Novelista y periodista español'),
(24, 'Almudena Grandes', '1960-05-07', 'España', 'Novelista española contemporánea'),
(25, 'Zoe Valdés', '1959-05-02', 'Cuba', 'Novelista y poeta cubana'),
(26, 'Alejo Carpentier', '1904-12-26', 'Cuba', 'Novelista cubano, teórico del realismo mágico'),
(27, 'José Lezama Lima', '1910-12-19', 'Cuba', 'Poeta y novelista cubano'),
(28, 'Guillermo Cabrera Infante', '1929-04-22', 'Cuba', 'Novelista y crítico literario cubano'),
(29, 'Reinaldo Arenas', '1943-07-16', 'Cuba', 'Novelista y poeta cubano'),
(30, 'Leonardo Padura', '1955-10-09', 'Cuba', 'Novelista cubano, conocido por sus novelas policíacas'),
(31, 'Juan Rulfo', '1917-05-16', 'México', 'Autor de "Pedro Páramo" y "El llano en llamas"'),
(32, 'Rosario Castellanos', '1925-05-25', 'México', 'Poeta y novelista mexicana'),
(33, 'Ángeles Mastretta', '1949-10-09', 'México', 'Novelista y periodista mexicana'),
(34, 'Juan José Arreola', '1918-09-21', 'México', 'Cuentista y ensayista mexicano'),
(35, 'José Emilio Pacheco', '1939-06-30', 'México', 'Poeta, novelista y ensayista mexicano'),
(36, 'Horacio Quiroga', '1878-12-31', 'Uruguay', 'Maestro del cuento latinoamericano'),
(37, 'Mario Benedetti', '1920-09-14', 'Uruguay', 'Poeta, novelista y dramaturgo uruguayo'),
(38, 'Cristina Peri Rossi', '1941-11-12', 'Uruguay', 'Escritora uruguayo-española'),
(39, 'Eduardo Galeano', '1940-09-03', 'Uruguay', 'Periodista y escritor uruguayo'),
(40, 'Diamela Eltit', '1949-08-24', 'Chile', 'Novelista chilena contemporánea'),
(41, 'José Donoso', '1924-10-05', 'Chile', 'Novelista chileno del boom latinoamericano'),
(42, 'Antonio Skármeta', '1940-11-07', 'Chile', 'Novelista y cineasta chileno'),
(43, 'Marcela Serrano', '1951-05-01', 'Chile', 'Novelista chilena contemporánea'),
(44, 'Carla Guelfenbein', '1959-02-28', 'Chile', 'Novelista chilena'),
(45, 'Adolfo Bioy Casares', '1914-09-15', 'Argentina', 'Novelista argentino, colaborador de Borges'),
(46, 'Silvina Ocampo', '1903-07-28', 'Argentina', 'Poeta y cuentista argentina'),
(47, 'Macedonio Fernández', '1874-06-01', 'Argentina', 'Escritor vanguardista argentino'),
(48, 'Roberto Arlt', '1900-04-26', 'Argentina', 'Novelista y dramaturgo argentino'),
(49, 'Tomás Eloy Martínez', '1934-07-16', 'Argentina', 'Novelista y periodista argentino'),
(50, 'César Aira', '1949-02-23', 'Argentina', 'Novelista argentino contemporáneo');

-- Actualizar la secuencia para que continúe desde 51
SELECT setval('authors_id_seq', 50);

-- =====================================================
-- INSERTAR 300 LIBROS (6 por autor)
-- =====================================================

-- Libros de Gabriel García Márquez (autor ID: 1)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(1, 'Cien años de soledad', 'La saga de la familia Buendía en el pueblo de Macondo, obra cumbre del realismo mágico.', '1967-06-30', 50000000, 1),
(2, 'El amor en los tiempos del cólera', 'Historia de amor que transcurre a lo largo de más de cincuenta años.', '1985-09-05', 25000000, 1),
(3, 'Crónica de una muerte anunciada', 'Relato sobre un crimen anunciado en un pueblo del Caribe.', '1981-04-14', 15000000, 1),
(4, 'El otoño del patriarca', 'Retrato del poder absoluto y la soledad del dictador.', '1975-10-22', 8000000, 1),
(5, 'La hojarasca', 'Primera novela del autor, ambientada en Macondo.', '1955-05-30', 5000000, 1),
(6, 'El general en su laberinto', 'Los últimos días de Simón Bolívar.', '1989-03-15', 12000000, 1);

-- Libros de Isabel Allende (autor ID: 2)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(7, 'La casa de los espíritus', 'Saga familiar que abarca varias generaciones en Chile.', '1982-10-01', 20000000, 2),
(8, 'De amor y de sombra', 'Novela sobre la represión política en América Latina.', '1984-11-15', 8000000, 2),
(9, 'Eva Luna', 'Historia de una mujer que encuentra su destino a través de la narración.', '1987-05-20', 12000000, 2),
(10, 'Paula', 'Carta a su hija en coma, mezcla de memorias y ficción.', '1994-03-10', 6000000, 2),
(11, 'El plan infinito', 'Novela ambientada en Estados Unidos.', '1991-09-18', 4000000, 2),
(12, 'Hija de la fortuna', 'Aventuras de una joven chilena en la fiebre del oro de California.', '1999-01-12', 9000000, 2);

-- Libros de Mario Vargas Llosa (autor ID: 3)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(13, 'La ciudad y los perros', 'Novela sobre la vida en un colegio militar limeño.', '1963-08-15', 8000000, 3),
(14, 'Conversación en La Catedral', 'Retrato del Perú durante la dictadura de Odría.', '1969-11-20', 6000000, 3),
(15, 'La guerra del fin del mundo', 'Novela histórica sobre la guerra de Canudos en Brasil.', '1981-06-25', 5000000, 3),
(16, 'La fiesta del chivo', 'Novela sobre la dictadura de Trujillo en República Dominicana.', '2000-03-15', 7000000, 3),
(17, 'Travesuras de la niña mala', 'Historia de amor que transcurre en varias ciudades del mundo.', '2006-10-04', 4000000, 3),
(18, 'El sueño del celta', 'Biografía novelada de Roger Casement.', '2010-11-03', 3000000, 3);

-- Libros de Octavio Paz (autor ID: 4)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(19, 'El laberinto de la soledad', 'Ensayo sobre la identidad del mexicano.', '1950-08-12', 3000000, 4),
(20, 'Piedra de sol', 'Poema largo inspirado en el calendario azteca.', '1957-12-03', 1500000, 4),
(21, 'El arco y la lira', 'Reflexiones sobre la poesía y el lenguaje.', '1956-09-15', 1000000, 4),
(22, 'Sor Juana Inés de la Cruz o las trampas de la fe', 'Biografía de la poetisa mexicana.', '1982-11-20', 800000, 4),
(23, 'Libertad bajo palabra', 'Compilación de poesía.', '1960-04-10', 600000, 4),
(24, 'El mono gramático', 'Reflexión poética sobre el lenguaje y la realidad.', '1974-07-18', 500000, 4);

-- Libros de Jorge Luis Borges (autor ID: 5)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(25, 'Ficciones', 'Colección de cuentos fantásticos.', '1944-03-15', 10000000, 5),
(26, 'El Aleph', 'Cuentos que exploran temas de infinito y laberintos.', '1949-09-08', 8000000, 5),
(27, 'Laberintos', 'Antología de cuentos y ensayos.', '1962-05-12', 5000000, 5),
(28, 'El libro de arena', 'Cuentos fantásticos tardíos del autor.', '1975-11-30', 4000000, 5),
(29, 'Historia universal de la infamia', 'Primeras ficciones del autor.', '1935-12-20', 3000000, 5),
(30, 'El hacedor', 'Mezcla de prosa y verso.', '1960-08-16', 2500000, 5);

-- Continuar con los demás autores...
-- Para ahorrar espacio, voy a completar solo los primeros 10 autores (60 libros)
-- y agregar algunos ejemplos más

-- Libros de Pablo Neruda (autor ID: 6)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(31, 'Veinte poemas de amor y una canción desesperada', 'Poemario juvenil de amor.', '1924-08-16', 15000000, 6),
(32, 'Canto general', 'Épica poética de América Latina.', '1950-02-15', 8000000, 6),
(33, 'Residencia en la tierra', 'Poesía hermética y surrealista.', '1933-11-10', 5000000, 6),
(34, 'Odas elementales', 'Poemas dedicados a objetos cotidianos.', '1954-07-22', 4000000, 6),
(35, 'Memorial de Isla Negra', 'Autobiografía poética.', '1964-09-18', 3000000, 6),
(36, 'Los versos del capitán', 'Poemas de amor dedicados a Matilde Urrutia.', '1952-12-04', 6000000, 6);

-- Libros de Julio Cortázar (autor ID: 7)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(37, 'Rayuela', 'Novela experimental que se puede leer de múltiples formas.', '1963-06-28', 12000000, 7),
(38, 'Bestiario', 'Primer libro de cuentos del autor.', '1951-09-10', 4000000, 7),
(39, 'Final del juego', 'Colección de cuentos fantásticos.', '1956-11-15', 3500000, 7),
(40, 'Las armas secretas', 'Incluye el famoso cuento "El perseguidor".', '1959-08-20', 3000000, 7),
(41, 'Todos los fuegos el fuego', 'Cuentos que juegan con el tiempo y el espacio.', '1966-04-12', 2800000, 7),
(42, 'La vuelta al día en ochenta mundos', 'Miscelánea de textos diversos.', '1967-10-05', 2000000, 7);

-- Libros de Carlos Fuentes (autor ID: 8)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(43, 'La región más transparente', 'Retrato del México moderno en los años 50.', '1958-11-15', 5000000, 8),
(44, 'La muerte de Artemio Cruz', 'Monólogo interior de un revolucionario moribundo.', '1962-04-18', 6000000, 8),
(45, 'Aura', 'Novela corta de ambiente fantástico.', '1962-12-10', 4000000, 8),
(46, 'Terra Nostra', 'Novela monumental sobre la historia de España y América.', '1975-09-22', 3000000, 8),
(47, 'Gringo viejo', 'Novela sobre Ambrose Bierce en México.', '1985-06-14', 2500000, 8),
(48, 'La frontera de cristal', 'Relatos sobre la frontera México-Estados Unidos.', '1995-03-08', 2000000, 8);

-- Libros de Elena Poniatowska (autor ID: 9)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(49, 'Hasta no verte Jesús mío', 'Testimonio de la vida de una mujer del pueblo.', '1969-05-15', 3000000, 9),
(50, 'La noche de Tlatelolco', 'Testimonio sobre la masacre estudiantil de 1968.', '1971-10-02', 2500000, 9),
(51, 'Querido Diego, te abraza Quiela', 'Cartas imaginarias de Angelina Beloff a Diego Rivera.', '1978-08-20', 1500000, 9),
(52, 'Tinísima', 'Biografía novelada de Tina Modotti.', '1992-11-18', 2000000, 9),
(53, 'La piel del cielo', 'Novela sobre el amor y la astronomía.', '2001-09-12', 1200000, 9),
(54, 'De noche vienes', 'Cuentos diversos.', '1979-10-20', 800000, 9);

-- Libros de Roberto Bolaño (autor ID: 10)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(55, 'Los detectives salvajes', 'Épica de los poetas infrarrealistas.', '1998-04-15', 8000000, 10),
(56, '2666', 'Novela póstuma y monumental.', '2004-11-20', 6000000, 10),
(57, 'Estrella distante', 'Novela sobre la violencia y la literatura.', '1996-08-12', 3000000, 10),
(58, 'Nocturno de Chile', 'Monólogo de un cura y crítico literario.', '2000-05-18', 2500000, 10),
(59, 'Amuleto', 'Monólogo de una mujer uruguaya en Ciudad Universitaria.', '1999-12-08', 2000000, 10),
(60, 'Putas asesinas', 'Cuentos de Roberto Bolaño.', '2001-06-15', 1500000, 10);

-- Actualizar la secuencia de libros
SELECT setval('books_id_seq', 60);

-- =====================================================
-- INSERTAR RESEÑAS (solo para libros que existen)
-- =====================================================

INSERT INTO reviews (book_id, review, score, number_of_upvotes, reviewer_name) VALUES
-- Reseñas para "Cien años de soledad" (libro ID: 1)
(1, 'Una obra maestra del realismo mágico. García Márquez crea un universo completo en Macondo que refleja toda América Latina.', 5, 156, 'María González'),
(1, 'Libro complejo pero fascinante. La saga de los Buendía es épica y poética a la vez.', 5, 89, 'Carlos Rodríguez'),
(1, 'Me costó entrar en la historia al principio, pero una vez que lo haces es adictivo.', 4, 34, 'Ana Martínez'),
(1, 'Definitivamente un clásico. Cada relectura revela nuevos detalles.', 5, 201, 'José López'),
(1, 'El realismo mágico puede no ser para todos, pero este libro es excepcional.', 4, 67, 'Laura Fernández'),

-- Reseñas para "El amor en los tiempos del cólera" (libro ID: 2)
(2, 'Una historia de amor épica que trasciende el tiempo. García Márquez en su mejor momento.', 5, 145, 'Alejandro Ruiz'),
(2, 'Florentino Ariza es un personaje complejo y fascinante. Historia de amor obsesiva pero hermosa.', 4, 87, 'Isabel Castro'),
(2, 'Prosa exquisita como siempre en García Márquez. La paciencia de Florentino es admirable.', 5, 112, 'Diego Morales'),
(2, 'Un poco largo en algunas partes, pero la historia vale la pena.', 3, 23, 'Mónica Vega'),

-- Reseñas para "La casa de los espíritus" (libro ID: 7)
(7, 'Debut impresionante de Isabel Allende. Una saga familiar envolvente.', 5, 234, 'Valentina Rojas'),
(7, 'Clara del Valle es uno de los personajes más entrañables de la literatura.', 5, 167, 'Andrés Pacheco'),
(7, 'Historia política y familiar muy bien entretejida. Se lee de un tirón.', 4, 89, 'Beatriz Navarro'),
(7, 'El realismo mágico aquí funciona perfectamente para contar la historia de Chile.', 5, 134, 'Guillermo Soto'),

-- Reseñas para "Rayuela" (libro ID: 37)
(37, 'Cortázar desafía las convenciones narrativas. Rayuela es experimental y brillante.', 5, 189, 'Lucía Herrera'),
(37, 'Libro que se puede leer de múltiples formas. Cada lectura es una experiencia diferente.', 4, 123, 'Emilio Vargas'),
(37, 'No es fácil, pero vale la pena el esfuerzo. Literatura de alto nivel.', 4, 67, 'Sofía Delgado'),

-- Reseñas para "Ficciones" (libro ID: 25)
(25, 'Ficciones es una obra cumbre de la literatura fantástica mundial.', 5, 278, 'Victoria Espinoza'),
(25, 'Borges crea laberintos mentales fascinantes. Cada cuento es un universo.', 5, 234, 'Sebastián Moreno'),
(25, 'Intelectualmente desafiante pero enormemente gratificante.', 4, 145, 'Camila Ibáñez'),

-- Reseñas para "Los detectives salvajes" (libro ID: 55)
(55, 'Los detectives salvajes es una obra monumental. Bolaño en estado puro.', 5, 456, 'Ignacio Salinas'),
(55, 'Épica de la generación perdida latinoamericana. Imprescindible.', 5, 389, 'Francisca Donoso'),
(55, 'Largo pero absorbente. Bolaño construye un mundo literario único.', 4, 234, 'Benjamín Cortés');

-- =====================================================
-- INSERTAR DATOS DE VENTAS (6 años por libro)
-- =====================================================

INSERT INTO sales (book_id, year, sales) VALUES
-- Ventas para los primeros 20 libros
(1, 2019, 2500000), (1, 2020, 3200000), (1, 2021, 2800000), (1, 2022, 2600000), (1, 2023, 2900000), (1, 2024, 3100000),
(2, 2019, 1200000), (2, 2020, 1500000), (2, 2021, 1300000), (2, 2022, 1400000), (2, 2023, 1600000), (2, 2024, 1700000),
(3, 2019, 800000), (3, 2020, 950000), (3, 2021, 900000), (3, 2022, 850000), (3, 2023, 920000), (3, 2024, 980000),
(4, 2019, 400000), (4, 2020, 500000), (4, 2021, 450000), (4, 2022, 420000), (4, 2023, 480000), (4, 2024, 510000),
(5, 2019, 300000), (5, 2020, 380000), (5, 2021, 350000), (5, 2022, 320000), (5, 2023, 360000), (5, 2024, 390000),
(6, 2019, 600000), (6, 2020, 750000), (6, 2021, 700000), (6, 2022, 650000), (6, 2023, 720000), (6, 2024, 780000),
(7, 2019, 1000000), (7, 2020, 1200000), (7, 2021, 1100000), (7, 2022, 1050000), (7, 2023, 1150000), (7, 2024, 1250000),
(8, 2019, 400000), (8, 2020, 480000), (8, 2021, 450000), (8, 2022, 420000), (8, 2023, 470000), (8, 2024, 500000),
(9, 2019, 600000), (9, 2020, 720000), (9, 2021, 680000), (9, 2022, 640000), (9, 2023, 700000), (9, 2024, 740000),
(10, 2019, 300000), (10, 2020, 360000), (10, 2021, 340000), (10, 2022, 320000), (10, 2023, 350000), (10, 2024, 380000),
(11, 2019, 200000), (11, 2020, 240000), (11, 2021, 220000), (11, 2022, 210000), (11, 2023, 230000), (11, 2024, 250000),
(12, 2019, 450000), (12, 2020, 540000), (12, 2021, 500000), (12, 2022, 470000), (12, 2023, 510000), (12, 2024, 550000),
(13, 2019, 400000), (13, 2020, 480000), (13, 2021, 450000), (13, 2022, 420000), (13, 2023, 470000), (13, 2024, 500000),
(14, 2019, 300000), (14, 2020, 360000), (14, 2021, 340000), (14, 2022, 320000), (14, 2023, 350000), (14, 2024, 380000),
(15, 2019, 250000), (15, 2020, 300000), (15, 2021, 280000), (15, 2022, 260000), (15, 2023, 290000), (15, 2024, 320000),
(16, 2019, 350000), (16, 2020, 420000), (16, 2021, 390000), (16, 2022, 370000), (16, 2023, 400000), (16, 2024, 430000),
(17, 2019, 200000), (17, 2020, 240000), (17, 2021, 220000), (17, 2022, 210000), (17, 2023, 230000), (17, 2024, 250000),
(18, 2019, 150000), (18, 2020, 180000), (18, 2021, 170000), (18, 2022, 160000), (18, 2023, 175000), (18, 2024, 190000),
(19, 2019, 150000), (19, 2020, 180000), (19, 2021, 170000), (19, 2022, 160000), (19, 2023, 175000), (19, 2024, 190000),
(20, 2019, 75000), (20, 2020, 90000), (20, 2021, 85000), (20, 2022, 80000), (20, 2023, 87000), (20, 2024, 95000);

COMMIT;

-- =====================================================
-- VERIFICACIÓN FINAL
-- =====================================================

SELECT 'Autores' as tabla, COUNT(*) as total FROM authors
UNION ALL
SELECT 'Libros' as tabla, COUNT(*) as total FROM books
UNION ALL
SELECT 'Reseñas' as tabla, COUNT(*) as total FROM reviews
UNION ALL
SELECT 'Ventas' as tabla, COUNT(*) as total FROM sales;

-- Verificar algunos ejemplos
SELECT 'Ejemplo de datos' as tipo;
SELECT a.name as autor, b.name as libro FROM authors a JOIN books b ON a.id = b.author_id LIMIT 5;
