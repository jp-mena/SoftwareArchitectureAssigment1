#!/bin/bash

# Script para generar certificados SSL para app.localhost
# Este script crea un certificado autofirmado para desarrollo local

echo "Generando certificado SSL para app.localhost..."

# Crear directorio para certificados si no existe
mkdir -p ssl

# Generar clave privada
openssl genrsa -out ssl/app.localhost.key 2048

# Generar certificado autofirmado
openssl req -new -x509 -key ssl/app.localhost.key -out ssl/app.localhost.crt -days 365 -subj "/C=CL/ST=Santiago/L=Santiago/O=BookReviews/OU=IT/CN=app.localhost"

# Crear archivo PEM combinado para HAProxy
cat ssl/app.localhost.crt ssl/app.localhost.key > ssl/app.localhost.pem

echo "Certificados generados en el directorio ssl/"
echo "Para usar con HAProxy, copia ssl/app.localhost.pem a /etc/ssl/certs/ en el contenedor"
echo ""
echo "Para que funcione en tu navegador, añade esta línea a tu archivo hosts:"
echo "127.0.0.1 app.localhost"
echo ""
echo "Ubicación del archivo hosts:"
echo "- Windows: C:\\Windows\\System32\\drivers\\etc\\hosts"
echo "- macOS/Linux: /etc/hosts"
