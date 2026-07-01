# syntax=docker/dockerfile:1
# ── Build ────────────────────────────────────────────────────────────────────
FROM rust:1.94-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev cmake perl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# CLI Runique
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install runique --version "=2.1.21" --features "orm,postgres" && \
    find /usr/local/cargo/registry -path "*/runique-2.1.21/static" -type d -exec cp -r {} /tmp/runique-static \;

# Build de l'app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    DB_ENGINE=postgres cargo build --release && \
    cp target/release/campanile /tmp/campanile

# Build migration
COPY migration ./migration
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/migration/target \
    DB_ENGINE=postgres cargo build --release --manifest-path migration/Cargo.toml && \
    cp migration/target/release/campanile-migration /tmp/campanile-migration

# ── Runtime ──────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /tmp/campanile ./
COPY --from=builder /tmp/campanile-migration ./migrate
COPY --from=builder /usr/local/cargo/bin/runique ./runique
COPY --from=builder /tmp/runique-static ./runique-static
COPY static ./static
COPY templates ./templates
RUN mkdir -p media

ENV BASE_DIR=/app

EXPOSE 3001

CMD ["./campanile"]
