# Étape 1 : Construction de l'application
FROM rust:latest as builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers nécessaires
COPY . .

# Installer les dépendances et compiler en mode release
RUN cargo build --release

# Étape 2 : Image finale minimale
FROM debian:bullseye-slim

# Installer uniquement les dépendances nécessaires pour PostgreSQL
RUN apt-get update && \
    apt-get install -y libpq5 && \
    apt-get install -y libc6 &&\
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

FROM debian:bullseye-slim

# Installer les dépendances nécessaires
RUN apt-get update && apt-get install -y wget build-essential manpages-dev

# Télécharger et compiler GLIBC
WORKDIR /tmp
RUN wget http://ftp.gnu.org/gnu/libc/glibc-2.34.tar.gz && \
    tar -xvzf glibc-2.34.tar.gz && \
    cd glibc-2.34 && \
    mkdir build && cd build && \
    ../configure --prefix=/opt/glibc-2.34 && \
    make -j$(nproc) && make install

# Mettre à jour le chemin pour utiliser GLIBC 2.34
ENV LD_LIBRARY_PATH=/opt/glibc-2.34/lib:$LD_LIBRARY_PATH


# Définir le répertoire de travail
WORKDIR /app

# Copier le binaire compilé depuis l'étape de construction
COPY --from=builder /app/target/release/HappytoBeesbackend ./api

# Exposer le port utilisé par l'API
EXPOSE 3000

# Commande pour démarrer l'application
CMD ["./api"]

