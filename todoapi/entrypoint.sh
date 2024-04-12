#!/bin/bash
set -e  # Arrêter le script en cas d'erreur

# Attendre que la base de données soit prête
echo "Waiting for PostgreSQL to start..."
while ! pg_isready -h postgres -p 5432 -U docker; do
    sleep 1;
done

echo "PostgreSQL started"

# Exécuter les migrations
echo "Running Diesel migrations"
diesel migration run --database-url postgresql://docker:password@postgres:5432/todolist

# Démarrer l'application principale
echo "Starting main application"
exec "$@"

