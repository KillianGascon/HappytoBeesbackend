ARG RUST_VERSION=1.80.0

FROM rust:${RUST_VERSION} as builder-rust
WORKDIR /app
COPY .env ./.env
RUN --mount=type=bind,source=HappytoBeesbackend,target=/app/HappytoBeesbackend,rw \
    --mount=type=cache,target=/app/target/,rw \
    --mount=type=cache,target=/usr/local/cargo/registry/,rw \
    sh -c " \
        set -e; \
        cd /app/HappytoBeesbackend; \
        cargo build --locked --release; \
        cp ./target/release/HappytoBeesbackend /; \
    "
    
FROM debian:12
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*
USER appuser
COPY --from=builder-rust /HappytoBeesbackend /

# Commande pour lancer l'HappytoBeesbackend
CMD /HappytoBeesbackend
