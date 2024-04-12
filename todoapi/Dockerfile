FROM rust:latest

# Définir le répertoire de travail
WORKDIR /usr/src/app

# Mettre à jour les paquets et installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# Copier le code source dans l'image
COPY . .

# Compiler l'application
RUN cargo build --release

# Installer diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Copier le script d'entrée et le rendre exécutable
COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

# Configurer l'entrypoint et la commande par défaut
ENTRYPOINT ["entrypoint.sh"]
CMD ["./target/release/todoapi"]


