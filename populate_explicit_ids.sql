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

-- Libros de Laura Esquivel (autor ID: 11)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(61, 'Como agua para chocolate', 'Historia de amor y tradición mexicana.', '1989-09-15', 5000000, 11),
(62, 'La ley del amor', 'Novela futurista con mezcla de géneros.', '1995-05-10', 1200000, 11),
(63, 'Tan veloz como el deseo', 'Relato de un telégrafo y su vida.', '2001-04-02', 800000, 11),
(64, 'Malinche', 'Historia de la intérprete de Hernán Cortés.', '2006-03-20', 700000, 11),
(65, 'A Lupita le gustaba planchar', 'Novela de una policía en la Ciudad de México.', '2014-08-01', 500000, 11);

-- Libros de Gioconda Belli (autor ID: 12)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(66, 'La mujer habitada', 'Novela feminista y política.', '1988-06-12', 1500000, 12),
(67, 'Sofía de los presagios', 'Historia de una mujer mística.', '1990-11-03', 800000, 12),
(68, 'El infinito en la palma de la mano', 'Premio Biblioteca Breve 2008.', '2008-05-15', 600000, 12),
(69, 'El país bajo mi piel', 'Memorias de revolución y vida.', '2001-09-25', 400000, 12),
(70, 'Waslala', 'Novela de ficción utópica.', '1996-03-22', 300000, 12);

-- Libros de Augusto Roa Bastos (autor ID: 13)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(71, 'Yo el Supremo', 'Novela sobre la dictadura paraguaya.', '1974-05-18', 2000000, 13),
(72, 'Hijo de hombre', 'Narración social paraguaya.', '1960-04-10', 1200000, 13),
(73, 'Contravida', 'Reflexión sobre el exilio y la memoria.', '1994-02-15', 500000, 13),
(74, 'El fiscal', 'Novela sobre dictaduras latinoamericanas.', '1993-06-30', 400000, 13),
(75, 'Madama Sui', 'Historia ambientada en la Guerra del Chaco.', '1995-10-21', 300000, 13);

-- Libros de Clarice Lispector (autor ID: 14)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(76, 'La pasión según G.H.', 'Exploración existencial de una mujer.', '1964-07-01', 900000, 14),
(77, 'Cerca del corazón salvaje', 'Debut literario de la autora.', '1943-12-15', 700000, 14),
(78, 'La hora de la estrella', 'Historia de Macabéa, una joven nordestina.', '1977-10-01', 1200000, 14),
(79, 'Un soplo de vida', 'Novela póstuma filosófica.', '1978-01-20', 500000, 14),
(80, 'Agua viva', 'Novela lírica y experimental.', '1973-03-10', 400000, 14);

-- Libros de Ernesto Sabato (autor ID: 15)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(81, 'El túnel', 'Novela psicológica y existencialista.', '1948-05-15', 2000000, 15),
(82, 'Sobre héroes y tumbas', 'Incluye el famoso "Informe sobre ciegos".', '1961-06-01', 1800000, 15),
(83, 'Abaddón el exterminador', 'Obra compleja y apocalíptica.', '1974-09-10', 1000000, 15),
(84, 'Uno y el universo', 'Ensayos filosóficos.', '1945-03-12', 600000, 15),
(85, 'Hombres y engranajes', 'Ensayo sobre la sociedad moderna.', '1951-04-22', 400000, 15);

-- Libros de José Saramago (autor ID: 16)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(86, 'Ensayo sobre la ceguera', 'Metáfora sobre la condición humana.', '1995-09-15', 3000000, 16),
(87, 'El evangelio según Jesucristo', 'Visión heterodoxa de la vida de Cristo.', '1991-11-10', 2000000, 16),
(88, 'Las intermitencias de la muerte', 'La muerte se detiene en un país.', '2005-05-15', 1200000, 16),
(89, 'Caín', 'Relectura bíblica irreverente.', '2009-10-30', 900000, 16),
(90, 'La balsa de piedra', 'La península ibérica se separa de Europa.', '1986-06-12', 800000, 16);

-- Libros de Fernando Pessoa (autor ID: 17)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(91, 'Libro del desasosiego', 'Obra fragmentaria y filosófica.', '1982-03-01', 1500000, 17),
(92, 'Mensagem', 'Único libro en portugués publicado en vida.', '1934-12-01', 800000, 17),
(93, 'Poesías de Álvaro de Campos', 'Heterónimo modernista.', '1924-05-15', 500000, 17),
(94, 'Poesías de Ricardo Reis', 'Obra clásica y contenida.', '1935-07-01', 400000, 17),
(95, 'Poesías de Alberto Caeiro', 'Versos pastoriles y naturales.', '1915-10-10', 600000, 17);

-- Libros de Camilo José Cela (autor ID: 18)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(96, 'La familia de Pascual Duarte', 'Novela tremendista española.', '1942-06-10', 2500000, 18),
(97, 'La colmena', 'Retrato coral del Madrid de posguerra.', '1951-02-15', 2000000, 18),
(98, 'Mazurca para dos muertos', 'Novela de la Guerra Civil.', '1983-09-20', 900000, 18),
(99, 'Viaje a la Alcarria', 'Crónica de viaje.', '1948-05-12', 700000, 18),
(100, 'Cristo versus Arizona', 'Novela experimental.', '1988-01-25', 500000, 18);

-- Libros de Ana María Matute (autor ID: 19)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(101, 'Los hijos muertos', 'Novela sobre la posguerra española.', '1958-04-01', 1200000, 19),
(102, 'Primera memoria', 'Premio Nadal 1959.', '1959-05-15', 1000000, 19),
(103, 'Los soldados lloran de noche', 'Segunda parte de la trilogía.', '1964-03-20', 800000, 19),
(104, 'La trampa', 'Tercera parte de la trilogía.', '1969-10-10', 700000, 19),
(105, 'Olvidado rey Gudú', 'Novela fantástica.', '1996-06-25', 600000, 19);

-- Libros de Carmen Laforet (autor ID: 20)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(106, 'Nada', 'Novela ganadora del Premio Nadal.', '1945-01-06', 2000000, 20),
(107, 'La isla y los demonios', 'Historia de adolescencia en Canarias.', '1952-04-10', 800000, 20),
(108, 'La mujer nueva', 'Exploración espiritual y moral.', '1955-06-12', 700000, 20),
(109, 'La insolación', 'Relato breve y psicológico.', '1963-02-01', 300000, 20),
(110, 'Paralelo 35', 'Cuentos publicados en su juventud.', '1967-05-20', 200000, 20);

-- Libros de Miguel Delibes (autor ID: 21)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(111, 'La sombra del ciprés es alargada', 'Primera novela, Premio Nadal 1947.', '1948-05-20', 900000, 21),
(112, 'El camino', 'Relato de la infancia en un pueblo castellano.', '1950-06-01', 1200000, 21),
(113, 'Las ratas', 'Denuncia social ambientada en la España rural.', '1962-09-15', 800000, 21),
(114, 'Cinco horas con Mario', 'Monólogo de una mujer ante el cadáver de su esposo.', '1966-03-05', 2000000, 21),
(115, 'Los santos inocentes', 'Novela sobre desigualdad social en el campo.', '1981-04-10', 2500000, 21);

-- Libros de Javier Marías (autor ID: 22)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(116, 'Corazón tan blanco', 'Historia sobre secretos y matrimonios.', '1992-10-15', 2000000, 22),
(117, 'Mañana en la batalla piensa en mí', 'Reflexión sobre la muerte y la culpa.', '1994-06-12', 1500000, 22),
(118, 'Tu rostro mañana I: Fiebre y lanza', 'Primera parte de la trilogía.', '2002-04-10', 1000000, 22),
(119, 'Tu rostro mañana II: Baile y sueño', 'Segunda parte de la trilogía.', '2004-05-20', 900000, 22),
(120, 'Tu rostro mañana III: Veneno y sombra y adiós', 'Tercera parte de la trilogía.', '2007-06-30', 850000, 22);

-- Libros de Arturo Pérez-Reverte (autor ID: 23)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(121, 'El club Dumas', 'Novela de misterio literario.', '1993-10-01', 1800000, 23),
(122, 'La tabla de Flandes', 'Intriga histórica y ajedrecística.', '1990-05-15', 1500000, 23),
(123, 'La piel del tambor', 'Thriller sobre un hacker y el Vaticano.', '1995-11-03', 1200000, 23),
(124, 'La reina del sur', 'Historia de una narcotraficante mexicana.', '2002-06-15', 3000000, 23),
(125, 'El capitán Alatriste', 'Primera novela de la saga de espadachines.', '1996-03-20', 2500000, 23);

-- Libros de Almudena Grandes (autor ID: 24)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(126, 'Las edades de Lulú', 'Novela erótica ganadora del Premio La Sonrisa Vertical.', '1989-05-12', 800000, 24),
(127, 'Malena es un nombre de tango', 'Historia de una mujer en busca de identidad.', '1994-04-20', 900000, 24),
(128, 'Atlas de geografía humana', 'Relatos entrelazados de cuatro mujeres.', '1998-03-25', 1000000, 24),
(129, 'El corazón helado', 'Saga familiar en la España del siglo XX.', '2007-09-12', 1500000, 24),
(130, 'Los pacientes del doctor García', 'Novela histórica sobre la posguerra.', '2017-09-20', 1200000, 24);

-- Libros de Zoe Valdés (autor ID: 25)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(131, 'La nada cotidiana', 'Novela sobre la vida en Cuba bajo el régimen.', '1995-02-15', 700000, 25),
(132, 'Te di la vida entera', 'Historia de amor y desarraigo.', '1996-04-20', 600000, 25),
(133, 'Querido primer novio', 'Relato autobiográfico y novelado.', '1999-07-10', 400000, 25),
(134, 'La eternidad del instante', 'Novela histórica ambientada en China.', '2004-10-01', 500000, 25),
(135, 'El todo cotidiano', 'Continuación espiritual de La nada cotidiana.', '2010-05-20', 300000, 25);

-- Libros de Alejo Carpentier (autor ID: 26)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(136, 'El reino de este mundo', 'Novela sobre la revolución haitiana.', '1949-06-12', 1200000, 26),
(137, 'Los pasos perdidos', 'Viaje iniciático en América Latina.', '1953-05-15', 1500000, 26),
(138, 'El siglo de las luces', 'Novela sobre la Revolución Francesa en el Caribe.', '1962-07-20', 1100000, 26),
(139, 'Concierto barroco', 'Novela corta sobre música y mestizaje.', '1974-09-01', 600000, 26),
(140, 'La consagración de la primavera', 'Novela sobre la Guerra Civil española.', '1978-05-10', 700000, 26);

-- Libros de José Lezama Lima (autor ID: 27)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(141, 'Paradiso', 'Novela compleja y poética.', '1966-08-15', 900000, 27),
(142, 'Oppiano Licario', 'Novela inacabada y publicada póstumamente.', '1977-02-01', 400000, 27),
(143, 'La expresión americana', 'Ensayo sobre cultura y arte.', '1957-05-20', 300000, 27),
(144, 'Dador', 'Poemario clave en su obra.', '1960-11-01', 250000, 27),
(145, 'Enemigo rumor', 'Primer libro de poemas.', '1941-03-15', 200000, 27);

-- Libros de Guillermo Cabrera Infante (autor ID: 28)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(146, 'Tres tristes tigres', 'Novela experimental sobre La Habana nocturna.', '1967-04-15', 1000000, 28),
(147, 'La Habana para un infante difunto', 'Novela autobiográfica y erótica.', '1979-07-20', 800000, 28),
(148, 'Cuerpos divinos', 'Novela sobre política y deseo.', '1981-09-10', 600000, 28),
(149, 'Mea Cuba', 'Crítica y reflexión sobre Cuba.', '1992-02-15', 500000, 28),
(150, 'Vidas para leerlas', 'Retratos de escritores y artistas.', '1998-10-05', 400000, 28);

-- Libros de Reinaldo Arenas (autor ID: 29)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(151, 'Celestino antes del alba', 'Primera novela de Arenas.', '1967-03-10', 600000, 29),
(152, 'El mundo alucinante', 'Novela sobre Fray Servando Teresa de Mier.', '1969-08-01', 700000, 29),
(153, 'Otra vez el mar', 'Novela censurada en Cuba.', '1982-04-05', 500000, 29),
(154, 'Antes que anochezca', 'Autobiografía del autor.', '1992-06-12', 1200000, 29),
(155, 'El color del verano', 'Novela satírica y política.', '1991-09-10', 400000, 29);

-- Libros de Leonardo Padura (autor ID: 30)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(156, 'Pasado perfecto', 'Primera novela de la serie de Mario Conde.', '1991-02-01', 800000, 30),
(157, 'Vientos de cuaresma', 'Segunda entrega de Mario Conde.', '1994-03-15', 750000, 30),
(158, 'Máscaras', 'Tercera novela policíaca de la saga.', '1997-05-12', 700000, 30),
(159, 'Paisaje de otoño', 'Cuarta novela de Mario Conde.', '1998-10-20', 680000, 30),
(160, 'El hombre que amaba a los perros', 'Novela sobre el asesinato de Trotski.', '2009-06-01', 1500000, 30);

-- Libros de Juan Rulfo (autor ID: 31)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(161, 'Pedro Páramo', 'Novela sobre la memoria y los fantasmas de Comala.', '1955-06-05', 2500000, 31),
(162, 'El llano en llamas', 'Colección de cuentos sobre la vida rural mexicana.', '1953-03-15', 1500000, 31),
(163, 'Diles que no me maten', 'Cuento sobre venganza y justicia.', '1953-04-20', 400000, 31),
(164, 'No oyes ladrar los perros', 'Relato sobre un padre y su hijo enfermo.', '1953-05-10', 350000, 31),
(165, 'Es que somos muy pobres', 'Cuento sobre la pobreza y la pérdida familiar.', '1953-06-12', 300000, 31);

-- Libros de Rosario Castellanos (autor ID: 32)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(166, 'Balún Canán', 'Novela sobre la opresión indígena en Chiapas.', '1957-09-10', 700000, 32),
(167, 'Oficio de tinieblas', 'Novela histórica sobre la conquista de México.', '1962-05-12', 650000, 32),
(168, 'Mujer que sabe latín...', 'Colección de cuentos y ensayos feministas.', '1973-04-05', 300000, 32),
(169, 'Poesía no eres tú', 'Libro de poemas sobre identidad y feminismo.', '1976-01-15', 250000, 32),
(170, 'Ciudad Real', 'Novela sobre la vida social y política mexicana.', '1960-06-20', 500000, 32);

-- Libros de Ángeles Mastretta (autor ID: 33)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(171, 'Arráncame la vida', 'Novela sobre la vida de una mujer en Puebla.', '1985-04-12', 1200000, 33),
(172, 'Mujeres de ojos grandes', 'Cuentos centrados en mujeres mexicanas.', '1990-06-10', 900000, 33),
(173, 'Mal de amores', 'Novela histórica y romántica sobre la Revolución mexicana.', '1996-08-15', 700000, 33),
(174, 'El mundo iluminado', 'Relatos cortos sobre la identidad femenina.', '2004-03-10', 400000, 33),
(175, 'Nervio óptico', 'Colección de cuentos breves.', '2007-05-20', 350000, 33);

-- Libros de Juan José Arreola (autor ID: 34)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(176, 'Confabulario', 'Colección de cuentos fantásticos y humorísticos.', '1952-01-10', 500000, 34),
(177, 'La feria', 'Novela sobre personajes y situaciones absurdas.', '1963-05-12', 400000, 34),
(178, 'Bestiario', 'Cuentos cortos sobre lo fantástico y lo grotesco.', '1959-03-20', 450000, 34),
(179, 'Palíndromos', 'Juego literario y reflexión sobre el lenguaje.', '1969-08-01', 300000, 34),
(180, 'El juguete rabioso', 'Obra sobre la juventud y la rebeldía.', '1946-06-15', 350000, 34);

-- Libros de José Emilio Pacheco (autor ID: 35)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(181, 'Las batallas en el desierto', 'Novela corta sobre la infancia y la sociedad mexicana.', '1981-02-10', 900000, 35),
(182, 'El principio del placer', 'Cuentos sobre el deseo y la sociedad urbana.', '1972-05-12', 400000, 35),
(183, 'El silencio de la luna', 'Colección de poesía y narrativa breve.', '1966-03-15', 300000, 35),
(184, 'No me preguntes cómo pasa el tiempo', 'Poemas sobre el tiempo y la memoria.', '1999-09-20', 500000, 35),
(185, 'El viento distante', 'Antología de cuentos seleccionados.', '2002-04-01', 350000, 35);

-- Libros de Horacio Quiroga (autor ID: 36)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(186, 'Cuentos de la selva', 'Relatos para niños sobre la vida en Misiones.', '1918-03-12', 700000, 36),
(187, 'Anaconda', 'Cuento sobre la violencia y la naturaleza.', '1921-05-20', 400000, 36),
(188, 'La gallina degollada', 'Cuento sobre tragedia familiar.', '1935-06-10', 450000, 36),
(189, 'Los desterrados', 'Relatos sobre exilio y muerte.', '1926-04-15', 350000, 36),
(190, 'El almohadón de plumas', 'Cuento de terror psicológico.', '1907-10-01', 500000, 36);

-- Libros de Mario Benedetti (autor ID: 37)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(191, 'La tregua', 'Novela sobre la vida cotidiana y el amor tardío.', '1960-09-15', 1800000, 37),
(192, 'Gracias por el fuego', 'Novela sobre la corrupción y la familia.', '1965-03-10', 900000, 37),
(193, 'Primavera con una esquina rota', 'Historias de amor y política en Uruguay.', '1982-05-12', 700000, 37),
(194, 'El cumpleaños de Juan Ángel', 'Novela sobre la memoria y la culpa.', '1971-07-20', 500000, 37),
(195, 'Andamios', 'Cuentos cortos sobre la vida urbana.', '1996-02-01', 350000, 37);

-- Libros de Cristina Peri Rossi (autor ID: 38)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(196, 'La nave de los locos', 'Novela sobre la sociedad contemporánea.', '1984-03-10', 400000, 38),
(197, 'Citas y visitas', 'Colección de cuentos sobre relaciones humanas.', '1987-05-15', 350000, 38),
(198, 'La ciudad y los perros', 'Ensayo sobre dictaduras y poder.', '1990-06-12', 300000, 38),
(199, 'Poemas de amor', 'Antología poética sobre el amor y la pasión.', '1979-09-01', 250000, 38),
(200, 'Solos y solas', 'Relatos sobre aislamiento y soledad.', '2005-11-10', 200000, 38);

-- Libros de Eduardo Galeano (autor ID: 39)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(201, 'Las venas abiertas de América Latina', 'Ensayo histórico sobre la explotación de América Latina.', '1971-03-15', 2000000, 39),
(202, 'Memoria del fuego I: Los nacimientos', 'Primera parte de la trilogía histórica.', '1982-06-01', 700000, 39),
(203, 'Memoria del fuego II: Las caras y las máscaras', 'Segunda parte de la trilogía histórica.', '1986-04-12', 600000, 39),
(204, 'Memoria del fuego III: El siglo del viento', 'Tercera parte de la trilogía histórica.', '1989-09-10', 500000, 39),
(205, 'El libro de los abrazos', 'Colección de relatos breves y reflexivos.', '1989-05-20', 900000, 39);

-- Libros de Diamela Eltit (autor ID: 40)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(206, 'Lumpérica', 'Novela experimental y feminista.', '1983-06-15', 300000, 40),
(207, 'Por la patria', 'Crítica social y política chilena.', '1986-09-20', 250000, 40),
(208, 'Los vigilantes', 'Relato sobre control y represión.', '1994-05-12', 200000, 40),
(209, 'El cuarto mundo', 'Novela sobre la desigualdad social.', '2000-07-01', 150000, 40),
(210, 'Impuestos imaginarios', 'Crítica política en clave literaria.', '2003-11-15', 100000, 40);

-- Libros de José Donoso (autor ID: 41)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(211, 'El obsceno pájaro de la noche', 'Novela emblemática del Boom latinoamericano.', '1970-03-10', 900000, 41),
(212, 'Coronación', 'Historia de la decadencia social en Chile.', '1957-09-15', 500000, 41),
(213, 'Casa de campo', 'Novela sobre la vida rural y urbana.', '1978-04-12', 400000, 41),
(214, 'El jardín de al lado', 'Novela psicológica y social.', '1981-07-20', 300000, 41),
(215, 'El lugar sin límites', 'Relato sobre sexualidad y poder en un pueblo.', '1966-05-10', 600000, 41),
(216, 'El mozo que salió de la tierra', 'Novela sobre identidad y violencia.', '1963-03-15', 350000, 41),
(217, 'El extranjero', 'Historia sobre alienación y soledad.', '1975-06-18', 250000, 41),
(218, 'Golpe de estado', 'Relato político y social.', '1985-02-12', 200000, 41),
(219, 'Donde van a morir los elefantes', 'Cuentos sobre la condición humana.', '1990-08-01', 180000, 41);

-- Libros de Antonio Skármeta (autor ID: 42)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(220, 'Ardiente paciencia', 'Novela sobre la amistad entre un poeta y un cartero.', '1985-03-15', 1200000, 42),
(221, 'El cartero de Neruda', 'Historia llevada al cine sobre Pablo Neruda.', '1985-06-20', 1500000, 42),
(222, 'No pasó nada', 'Novela sobre la dictadura chilena.', '1980-04-12', 400000, 42),
(223, 'El juego de Ripley', 'Novela sobre engaños y relaciones humanas.', '1990-07-01', 350000, 42),
(224, 'El baile de la Victoria', 'Novela sobre la vida en dictadura y exilio.', '2000-05-10', 450000, 42),
(225, 'La boda del poeta', 'Cuento sobre la vida amorosa y literaria.', '1995-08-12', 300000, 42),
(226, 'La montaña de los dioses', 'Novela histórica sobre Chile.', '1992-03-05', 250000, 42),
(227, 'El entusiasmo', 'Relato breve sobre pasión y creatividad.', '2002-01-15', 200000, 42),
(228, 'La ciudad donde envejeces', 'Novela contemporánea sobre memoria y tiempo.', '2010-04-20', 180000, 42);

-- Libros de Marcela Serrano (autor ID: 43)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(229, 'Nosotras que nos queremos tanto', 'Novela sobre la amistad femenina.', '1991-02-10', 1000000, 43),
(230, 'Para que no me olvides', 'Relatos sobre memoria y emociones.', '1993-06-12', 750000, 43),
(231, 'Diez mujeres', 'Historias sobre vidas de distintas mujeres.', '1997-09-15', 700000, 43),
(232, 'Antigua vida mía', 'Novela sobre amor y secretos familiares.', '1995-03-10', 600000, 43),
(233, 'El albergue de las mujeres tristes', 'Novela sobre dolor y sanación.', '2000-04-20', 500000, 43),
(234, 'La llorona', 'Relato contemporáneo sobre Chile.', '2003-05-15', 400000, 43),
(235, 'Hombre que mira al cielo', 'Novela sobre relaciones y búsqueda de sentido.', '2007-08-10', 350000, 43),
(236, 'Antología de cuentos', 'Colección de cuentos publicados anteriormente.', '2010-06-12', 300000, 43),
(237, 'Nosotras en la ciudad', 'Historias de mujeres en la vida urbana.', '2015-01-20', 250000, 43);

-- Libros de Carla Guelfenbein (autor ID: 44)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(238, 'El revés del alma', 'Novela sobre el amor y la traición.', '2002-03-10', 500000, 44),
(239, 'Café society', 'Relatos de vidas contemporáneas.', '2004-06-12', 400000, 44),
(240, 'Nadar desnudas', 'Historia de amistad y secretos.', '2006-08-15', 350000, 44),
(241, 'Contigo en la distancia', 'Novela romántica y familiar.', '2009-05-10', 300000, 44),
(242, 'La mujer de la libreta roja', 'Novela sobre recuerdos y pasiones.', '2011-04-20', 250000, 44),
(243, 'Al final del verano', 'Relato sobre cambios y decisiones.', '2013-07-01', 200000, 44),
(244, 'Quién te lo iba a decir', 'Novela sobre amor y pérdida.', '2015-09-12', 180000, 44),
(245, 'Lo que está perdido', 'Historia de encuentros y desencuentros.', '2018-03-15', 150000, 44),
(246, 'Tu nombre en mi boca', 'Novela sobre memoria y emociones.', '2020-01-10', 120000, 44);

-- Libros de Adolfo Bioy Casares (autor ID: 45)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(247, 'La invención de Morel', 'Novela de ciencia ficción y filosofía.', '1940-03-10', 900000, 45),
(248, 'Diario de la guerra del cerdo', 'Novela distópica sobre violencia y juventud.', '1969-05-12', 400000, 45),
(249, 'El sueño de los héroes', 'Novela sobre obsesión y memoria.', '1954-08-15', 350000, 45),
(250, 'Dormir al sol', 'Novela sobre amor y fantasía.', '1973-09-20', 300000, 45),
(251, 'Cuentos breves', 'Colección de relatos fantásticos.', '1946-06-12', 250000, 45),
(252, 'Los que aman, odian', 'Novela sobre pasiones humanas.', '1946-04-10', 200000, 45),
(253, 'El gran serafín', 'Relato fantástico y humorístico.', '1950-03-15', 180000, 45),
(254, 'Un campeón desparejo', 'Novela sobre deporte y sociedad.', '1955-07-01', 150000, 45),
(255, 'El lado de la sombra', 'Relato breve sobre identidad y misterio.', '1960-10-12', 120000, 45);

-- Libros de Silvina Ocampo (autor ID: 46)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(256, 'Viaje olvidado', 'Novela sobre memoria y ensoñación.', '1948-05-10', 300000, 46),
(257, 'Los que aman, odian', 'Relato sobre amor y conflictos.', '1946-06-12', 250000, 46),
(258, 'Cuentos de la niña que no existió', 'Colección de cuentos fantásticos.', '1953-03-15', 200000, 46),
(259, 'El solitario', 'Relato psicológico y oscuro.', '1961-04-20', 150000, 46),
(260, 'Cuando las horas se parecen', 'Cuentos sobre tiempo y memoria.', '1965-07-01', 120000, 46),
(261, 'La furia', 'Novela corta sobre la violencia.', '1968-09-12', 100000, 46),
(262, 'Los días de la noche', 'Relatos fantásticos y poéticos.', '1970-03-10', 90000, 46),
(263, 'Viaje a la muerte', 'Cuento sobre la obsesión y la fantasía.', '1975-06-15', 80000, 46),
(264, 'Los inocentes', 'Relatos de misterio y suspenso.', '1980-01-20', 70000, 46);

-- Libros de Macedonio Fernández (autor ID: 47)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(265, 'Museo de la novela de la eterna', 'Novela experimental y filosófica.', '1967-03-10', 500000, 47),
(266, 'Papeles de Recienvenido', 'Colección de textos vanguardistas.', '1920-06-12', 300000, 47),
(267, 'El mirador', 'Novela corta sobre la percepción y la vida.', '1927-08-15', 250000, 47),
(268, 'No toda es vigilia la de los ojos abiertos', 'Relato filosófico y experimental.', '1933-05-20', 200000, 47),
(269, 'Desde las sombras', 'Textos sobre reflexión y literatura.', '1942-04-10', 150000, 47),
(270, 'Los siete locos', 'Obra temprana vanguardista.', '1911-03-05', 120000, 47),
(271, 'Un ensayo de metafísica', 'Reflexión literaria y filosófica.', '1950-07-12', 100000, 47),
(272, 'Textos dispersos', 'Compilación de ensayos y cuentos breves.', '1960-06-15', 90000, 47),
(273, 'La novela y el lector', 'Ensayo sobre teoría de la novela.', '1965-01-20', 80000, 47);

-- Libros de Roberto Arlt (autor ID: 48)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(274, 'El juguete rabioso', 'Novela sobre juventud y marginalidad.', '1926-03-10', 600000, 48),
(275, 'Los siete locos', 'Novela sobre la crisis social y existencial.', '1929-05-12', 500000, 48),
(276, 'Los lanzallamas', 'Continuación de Los siete locos.', '1931-06-15', 450000, 48),
(277, 'Aguafuertes porteñas', 'Crónicas urbanas sobre Buenos Aires.', '1933-08-10', 350000, 48),
(278, 'El amor brujo', 'Novela corta sobre pasión y engaño.', '1932-04-12', 300000, 48),
(279, 'Historia de un profesional', 'Relato sobre la vida de un hombre común.', '1934-06-15', 250000, 48),
(280, 'El criador de gorilas', 'Cuentos sobre lo absurdo y lo grotesco.', '1930-02-20', 200000, 48),
(281, 'El desierto', 'Novela corta sobre soledad y angustia.', '1935-05-10', 150000, 48),
(282, 'Los lanzallamas II', 'Continuación y profundización de la serie.', '1936-09-15', 120000, 48);

-- Libros de Tomás Eloy Martínez (autor ID: 49)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(283, 'Santa Evita', 'Novela sobre la vida y mito de Eva Perón.', '1995-03-10', 1000000, 49),
(284, 'La novela de Perón', 'Obra histórica y literaria sobre Juan Perón.', '1985-06-12', 700000, 49),
(285, 'El vuelo de la reina', 'Novela corta sobre política y sociedad.', '1979-05-15', 400000, 49),
(286, 'Purgatorio', 'Relato sobre memoria y dictadura.', '1982-04-10', 350000, 49),
(287, 'La mano del amo', 'Novela histórica y política.', '1975-03-12', 300000, 49),
(288, 'El cantor de tango', 'Cuentos sobre música y tradición.', '1990-07-01', 250000, 49),
(289, 'La pasión de los argentinos', 'Novela histórica sobre conflictos nacionales.', '1988-01-15', 200000, 49);
--(290, 'Misterios de Buenos Aires', 'Relatos de intriga y política.', '1992-05-20', 49),
--(291, 'Historia de un sentimiento', 'Novela sobre amores y memorias argentinas.', '1998-09-10', 49);

-- Libros de César Aira (autor ID: 50)
INSERT INTO books (id, name, summary, date_of_publication, number_of_sales, author_id) VALUES
(292, 'La prueba', 'Novela breve y experimental.', '1983-03-10', 200000, 50),
(293, 'El llanto', 'Relato sobre misterio y absurdo.', '1985-06-12', 180000, 50),
(294, 'El tilo', 'Novela sobre la infancia y el tiempo.', '1989-05-15', 220000, 50),
(295, 'El santo', 'Historia fantástica y absurda.', '1992-04-10', 150000, 50),
(296, 'Fragmentos de un diario', 'Relato experimental sobre la memoria.', '1995-03-12', 120000, 50),
(297, 'La novela luminosa', 'Novela corta y poética.', '1997-07-01', 100000, 50),
(298, 'Cómo me hice monja', 'Relato humorístico y satírico.', '2000-01-15', 90000, 50),
(299, 'El cuarto de los objetos', 'Cuento sobre lo cotidiano y lo absurdo.', '2003-05-20', 85000, 50),
(300, 'El cerebro musical', 'Novela breve sobre obsesión y arte.', '2005-09-10', 80000, 50),
(301, 'La guerra de los gimnasios', 'Novela breve y absurda sobre rivalidades y obsesiones.', '2007-02-10', 75000, 50),
(302, 'El santo', 'Relato fantástico y satírico sobre la vida cotidiana.', '2009-06-15', 70000, 50);


-- Actualizar la secuencia de libros
SELECT setval('books_id_seq', 300);

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
