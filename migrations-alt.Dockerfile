# Alternative: Use Rust slim image with optimized build
FROM rust:1.75-slim-bookworm

WORKDIR /app

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli with minimal features and use sparse registry for faster builds
RUN cargo install sqlx-cli --version 0.7.3 --no-default-features --features postgres,rustls

# Copy migrations directory
COPY migrations ./migrations

# Set the default command to run migrations
CMD ["sqlx", "migrate", "run"]
