# --- Builder stage ---
    FROM rust:1-bookworm AS builder
    WORKDIR /app
    
    # Install build deps (openssl is optional; keep if your deps need it)
    RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
    
    # Leverage build cache
    COPY Cargo.toml Cargo.lock ./
    COPY src ./src
    # If you use SQLx offline, also:
    # COPY sqlx-data.json ./
    
    RUN cargo build --release
    
    # --- Runtime stage ---
    FROM debian:bookworm-slim AS runtime
    RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/* \
      && useradd -m -u 10001 appuser
    
    WORKDIR /app
    # Adjust binary name if your package/bin name differs
    COPY --from=builder /app/target/release/backend /usr/local/bin/app
    
    ENV RUST_LOG=info
    EXPOSE 8000
    USER appuser
    CMD ["/usr/local/bin/app"]