FROM rust:latest

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Instalar sqlx-cli para migraciones
RUN cargo install sqlx-cli

# Directorio de trabajo
WORKDIR /app

# Copiar todo el código
COPY . .

# Exponer puerto
EXPOSE 8000

# Hacer el script ejecutable
RUN chmod +x start.sh

# Script para ejecutar migraciones y luego la app
CMD ["./start.sh"]
