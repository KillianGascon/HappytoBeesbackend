# Étape 1 : Construction de l'application
FROM rust:latest AS builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers nécessaires
COPY . .

# Installer les dépendances et compiler en mode release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:bullseye-slim

# Installer les dépendances nécessaires
RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    ca-certificates \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Définir le répertoire de travail
WORKDIR /app

# Copier le binaire compilé depuis l'étape de construction
COPY --from=builder /app/target/release/HappytoBeesbackend ./api

# Exposer le port utilisé par l'API
EXPOSE 8080

# Commande pour démarrer l'application
CMD ["./api"]
