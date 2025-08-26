# Book Reviews API 📚

Una API REST construida con Rust y Rocket para gestionar reseñas de libros, conectada a PostgreSQL.

## 🚀 Inicio Rápido

### 1. Configuración
```bash
# 1. Copia el archivo de configuración
cp Rocket.toml.example Rocket.toml

# 2. Edita Rocket.toml con tus credenciales:
#    - Cambia TU_USUARIO por tu usuario de PostgreSQL
#    - Cambia TU_CONTRASEÑA por tu contraseña (codifica @ como %40)
#    - Cambia TU_BASE_DE_DATOS por el nombre de tu base de datos
#    - Genera nuevas secret_key con: openssl rand -base64 32
```

### 2. Ejecutar con Docker (Recomendado)
```bash
# Ejecutar aplicación y base de datos en contenedores
docker compose up --build

# La aplicación estará disponible en: http://localhost:8000
# PostgreSQL estará disponible en: localhost:5433
```

### 3. Ejecutar Localmente (Desarrollo)
```bash
# Asegúrate de tener PostgreSQL ejecutándose localmente
cargo run

# La aplicación estará disponible en: http://localhost:8000
```

## 🔧 Configuración del Proyecto

### Prerrequisitos

- **Docker y Docker Compose** (para la opción recomendada)
- **Rust** (última versión estable) + **PostgreSQL** (para desarrollo local)

### Configuración de la Base de Datos Local (Opcional)

### Configuración de la Base de Datos Local (Opcional)

Si quieres ejecutar PostgreSQL localmente en lugar de usar Docker:

1. **Crear la base de datos y usuario en PostgreSQL:**

```sql
-- Conectarse como superusuario (postgres)
CREATE USER bookuser WITH PASSWORD 'tu_password_aqui';
CREATE DATABASE bookreviews OWNER bookuser;
GRANT ALL PRIVILEGES ON DATABASE bookreviews TO bookuser;
```

2. **Ejecutar migraciones:**

```bash
# Si tienes sqlx-cli instalado
cargo install sqlx-cli
sqlx migrate run

# O manualmente ejecuta el archivo de migración en tu cliente PostgreSQL
```

## ⚙️ Configuración Detallada

### Archivo Rocket.toml

El archivo `Rocket.toml.example` contiene la configuración completa. Después de copiarlo a `Rocket.toml`, modifica:

- **Secret Keys**: Genera nuevas con `openssl rand -base64 32`
- **Database URLs**: Actualiza usuario, contraseña y nombre de base de datos
- **Perfiles**: `[default]` para local, `[docker]` para contenedores

### Variables de Entorno

También puedes usar variables de entorno en lugar de `Rocket.toml`:

```bash
export ROCKET_DATABASES='{book_db={url="postgres://usuario:contraseña@localhost:5432/bookreviews"}}'
export ROCKET_SECRET_KEY="tu-secret-key-aqui"
cargo run
```

## 📋 Rutas Disponibles

- **GET /** - Página de bienvenida
- **GET /health/db** - Verificar conexión a la base de datos

## 🛠️ Desarrollo

### Estructura del Proyecto

```
bookreviews/
├── src/
│   └── main.rs          # Punto de entrada principal
├── migrations/          # Migraciones de base de datos
├── Cargo.toml          # Dependencias del proyecto
├── Rocket.toml.example # Plantilla de configuración
└── README.md           # Este archivo
```

### Comandos Útiles

```bash
# Compilar sin ejecutar
cargo build

# Ejecutar en modo debug
cargo run

# Ejecutar tests
cargo test

# Compilar para producción
cargo build --release
```

## 🔒 Seguridad

- **NUNCA** subas el archivo `Rocket.toml` al repositorio
- Usa variables de entorno para producción
- Las credenciales de la base de datos deben mantenerse privadas

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -am 'Agregar nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## 🌱 Seed de la Base de Datos

Para poblar la base de datos con datos iniciales (seed), utiliza el archivo `populate_explicit_ids.sql`.  
Este archivo contiene las inserciones necesarias para tener información de prueba en tu aplicación.

Ejecuta el siguiente comando en tu terminal:

```bash
psql -h localhost -U <usuario_db> -d <nombre_db> -f populate_explicit_ids.sql
