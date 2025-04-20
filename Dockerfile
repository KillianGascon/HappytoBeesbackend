# Étape 1 : Construction de l'application
FROM rust:latest as builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers nécessaires
COPY . .

# Installer les dépendances et compiler en mode release
RUN cargo build --release

# Étape 2 : Image finale minimale avec Alpine Linux
FROM alpine:latest

# Installer GLIBC et PostgreSQL client
RUN apk add --no-cache \
    libc6-compat libpq \
    && apk del gcompat alpine-baselayout-data \
    && wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub \
    && wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.34-r0/glibc-2.34-r0.apk \
    && apk add --allow-untrusted ./glibc-2.34-r0.apk \
    && rm -f glibc-2.34-r0.apk


# Définir le répertoire de travail
WORKDIR /app

# Copier le binaire compilé depuis l'étape de construction
COPY --from=builder /app/target/release/HappytoBeesbackend ./api

# Exposer le port utilisé par l'API
EXPOSE 3000

# Commande pour démarrer l'application
CMD ["./api"]
