# Book Reviews - Assignment 3

## Descripción

Este proyecto implementa un sistema de reseñas de libros con las siguientes funcionalidades adicionales para la Asignación 3:

- **Caché Redis**: Cachea puntuaciones de reseñas, información de autores y consultas comunes
- **Motor de búsqueda OpenSearch**: Indexa resúmenes de libros y contenido de reseñas
- **Carga de imágenes**: Soporte para portadas de libros y fotos de autores
- **HAProxy**: Proxy inverso con dominio personalizado app.localhost
- **Múltiples despliegues**: Diferentes configuraciones Docker Compose

## Requisitos del Sistema

- Docker y Docker Compose
- Rust (para desarrollo local)
- OpenSSL (para generar certificados SSL)

## Configuraciones de Despliegue

### 1. Application + Database
```bash
docker-compose -f docker-compose.app-db.yml up -d
```

### 2. Application + Database + Cache
```bash
docker-compose -f docker-compose.app-db-cache.yml up -d
```

### 3. Application + Database + Search Engine
```bash
docker-compose -f docker-compose.app-db-search.yml up -d
```

### 4. Application + Database + Reverse Proxy
```bash
# Primero generar certificados SSL
./generate-ssl-cert.sh

# Añadir a /etc/hosts (Linux/macOS) o C:\Windows\System32\drivers\etc\hosts (Windows):
# 127.0.0.1 app.localhost

docker-compose -f docker-compose.app-db-proxy.yml up -d
```

### 5. Application + Database + Reverse Proxy + Cache + Search Engine (Completo)
```bash
# Primero generar certificados SSL
./generate-ssl-cert.sh

# Añadir a /etc/hosts:
# 127.0.0.1 app.localhost

docker-compose -f docker-compose.full.yml up -d
```

## Variables de Entorno

| Variable | Descripción | Valor por defecto |
|----------|-------------|-------------------|
| `DATABASE_URL` | URL de conexión a PostgreSQL | `postgresql://bookuser:Hol%401234@db:5432/bookreviews` |
| `REDIS_URL` | URL de conexión a Redis | `redis://redis:6379` |
| `OPENSEARCH_URL` | URL de conexión a OpenSearch | `http://opensearch:9200` |
| `SERVE_STATIC` | Si la aplicación debe servir archivos estáticos | `true` |
| `STATIC_PATH` | Ruta para archivos estáticos | `static` |
| `UPLOAD_PATH` | Ruta para archivos subidos | `uploads` |

## Funcionalidades Implementadas

### Caché Redis
- Cachea información de autores por 1 hora
- Cachea información de libros por 1 hora
- Cachea puntuaciones de reseñas por 30 minutos
- Cachea estadísticas de autores por 30 minutos
- Purga automática de caché cuando se modifican elementos

### Motor de Búsqueda OpenSearch
- Indexa resúmenes de libros y nombres de autores
- Indexa contenido completo de reseñas
- Búsqueda por texto libre en títulos y contenido
- Sincronización automática cuando se modifican elementos

### Carga de Imágenes
- Endpoint: `POST /api/upload/image`
- Formatos soportados: JPG, PNG, GIF, WebP
- Nombres únicos generados con UUID
- Organización por tipo de entidad (author/book)

### HAProxy
- Proxy inverso con balanceo de carga
- Servicio de archivos estáticos
- Redirección HTTP a HTTPS
- Estadísticas en http://app.localhost:8404/stats
- Configuración en `haproxy.cfg`

## Endpoints de la API

### Autores
- `GET /api/authors` - Listar autores
- `POST /api/authors` - Crear autor
- `GET /api/authors/{id}` - Obtener autor
- `PUT /api/authors/{id}` - Actualizar autor
- `DELETE /api/authors/{id}` - Eliminar autor

### Libros
- `GET /api/books` - Listar libros
- `POST /api/books` - Crear libro
- `GET /api/books/{id}` - Obtener libro
- `PUT /api/books/{id}` - Actualizar libro
- `DELETE /api/books/{id}` - Eliminar libro

### Reseñas
- `GET /api/reviews` - Listar reseñas
- `POST /api/reviews` - Crear reseña
- `GET /api/reviews/{id}` - Obtener reseña
- `PUT /api/reviews/{id}` - Actualizar reseña
- `DELETE /api/reviews/{id}` - Eliminar reseña

### Búsqueda
- `GET /api/search?q={query}` - Buscar en libros y reseñas

### Carga de Archivos
- `POST /api/upload/image` - Subir imagen (author/book)

## Monitoreo y Estadísticas

### HAProxy Stats
- URL: http://app.localhost:8404/stats
- Usuario: admin (sin contraseña)

### Health Check
- URL: http://app.localhost/health/db
- Verifica conexión a base de datos y existencia de tablas

## Pruebas de Carga

Para realizar pruebas de carga, puedes usar herramientas como:

- **Gatling**: https://gatling.io/
- **JMeter**: https://jmeter.apache.org/
- **Artillery**: https://artillery.io/

### Ejemplo con Artillery
```bash
npm install -g artillery
artillery quick --count 100 --num 10 http://app.localhost
```

## Estructura del Proyecto

```
SoftwareArchitectureAssigment1/
├── src/
│   ├── main.rs              # Punto de entrada principal
│   ├── models.rs            # Modelos de datos
│   ├── cache.rs             # Servicio de caché Redis
│   ├── search.rs            # Servicio de búsqueda OpenSearch
│   ├── static_files.rs      # Manejo de archivos estáticos
│   ├── file_upload.rs       # Carga de archivos
│   ├── authors.rs           # API de autores
│   ├── books.rs             # API de libros
│   ├── reviews.rs           # API de reseñas
│   └── ...
├── migrations/              # Migraciones de base de datos
├── templates/               # Plantillas HTML
├── docker-compose.*.yml     # Configuraciones de despliegue
├── haproxy.cfg              # Configuración de HAProxy
├── generate-ssl-cert.sh     # Script para certificados SSL
└── README_ASSIGNMENT3.md    # Este archivo
```

## Solución de Problemas

### Error de conexión a Redis
- Verificar que el contenedor Redis esté ejecutándose
- Comprobar la variable `REDIS_URL`

### Error de conexión a OpenSearch
- Verificar que el contenedor OpenSearch esté ejecutándose
- Comprobar la variable `OPENSEARCH_URL`
- Esperar a que OpenSearch termine de inicializar (puede tomar 1-2 minutos)

### Error de certificados SSL
- Ejecutar `./generate-ssl-cert.sh`
- Verificar que el archivo hosts contenga `127.0.0.1 app.localhost`

### Archivos estáticos no se cargan
- Verificar la variable `SERVE_STATIC`
- Comprobar que los directorios `static` y `uploads` existan
- Verificar permisos de archivos

## Desarrollo Local

Para desarrollo local sin Docker:

```bash
# Instalar dependencias
cargo build

# Configurar variables de entorno
export DATABASE_URL="postgresql://bookuser:Hol@1234@localhost:5433/bookreviews"
export REDIS_URL="redis://localhost:6379"  # Opcional
export OPENSEARCH_URL="http://localhost:9200"  # Opcional

# Ejecutar migraciones
sqlx migrate run

# Ejecutar aplicación
cargo run
```

## Notas Importantes

1. **Caché**: La aplicación funciona sin Redis, pero con mejor rendimiento con él
2. **Búsqueda**: La aplicación funciona sin OpenSearch, pero sin funcionalidad de búsqueda
3. **Proxy**: Con HAProxy, los archivos estáticos se sirven desde el proxy, no desde la aplicación
4. **SSL**: Los certificados son autofirmados, el navegador mostrará una advertencia de seguridad
5. **Dominio**: Asegúrate de añadir `app.localhost` a tu archivo hosts para que funcione correctamente
