#!/bin/bash

# Script para esperar a que la base de datos esté lista y luego ejecutar la aplicación

echo "Esperando a que la base de datos esté lista..."

# Función para verificar si la base de datos está lista
wait_for_db() {
    while ! pg_isready -h db -p 5432 -U bookuser -d bookreviews; do
        echo "Base de datos no está lista, esperando..."
        sleep 2
    done
    echo "Base de datos está lista!"
}

# Esperar a que la base de datos esté lista
wait_for_db

# Ejecutar migraciones
echo "Ejecutando migraciones..."
sqlx migrate run

# Ejecutar la aplicación
echo "Iniciando aplicación..."
cargo run
