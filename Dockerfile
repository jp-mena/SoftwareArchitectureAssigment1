FROM rust:latest

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Instalar sqlx-cli para migraciones
RUN cargo install sqlx-cli

# Directorio de trabajo
WORKDIR /app

# Copiar todo el código
COPY . .

# Exponer puerto
EXPOSE 8000

# Script para ejecutar migraciones y luego la app
CMD ["sh", "-c", "sleep 10 && sqlx migrate run && cargo run"]
