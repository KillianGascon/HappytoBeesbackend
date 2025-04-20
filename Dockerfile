# Étape 1 : Construction de l'application
FROM rust:latest as builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers nécessaires
COPY . .

# Installer les dépendances et compiler en mode release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:buster-slim

# Installer les dépendances minimales nécessaires
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Définir le répertoire de travail
WORKDIR /app

# Copier le binaire compilé depuis l'étape de construction
COPY --from=builder /app/target/release/HappytoBeesbackend ./api

# Exposer le port utilisé par l'API (Coolify utilise le port 8000 par défaut)
EXPOSE 8000

# Définir les variables d'environnement
ENV RUST_LOG=info
ENV PORT=8000

# Commande pour démarrer l'application
CMD ["./api"]

