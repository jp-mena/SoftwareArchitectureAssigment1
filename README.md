# Book Reviews API 📚

Una API REST construida con Rust y Rocket para gestionar reseñas de libros, conectada a PostgreSQL.

## 🚀 Configuración del Proyecto

### Prerrequisitos

- **Rust** (última versión estable)
- **PostgreSQL** (versión 12 o superior)
- **Cargo** (incluido con Rust)

### Configuración de la Base de Datos

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

### Configuración del Servidor

1. **Clonar el repositorio:**

```bash
git clone [tu-repo-url]
cd bookreviews
```

2. **Configurar Rocket.toml:**

```bash
# Copiar el archivo de ejemplo
cp Rocket.toml.example Rocket.toml

# Editar con tus credenciales reales
# Asegúrate de escapar caracteres especiales en la URL:
# @ se convierte en %40
# Por ejemplo: postgres://bookuser:Hol%401234@localhost/bookreviews
```

3. **Ejecutar el proyecto:**

```bash
cargo run
```

El servidor se iniciará en `http://localhost:8000`

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

## 📄 Licencia

Este proyecto está bajo la licencia MIT.
