FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/opsa

# Preserve the repository layout so compile-time paths match local builds.
COPY excretor ./excretor
COPY migrations ./migrations

WORKDIR /usr/src/opsa/excretor

# Build the binary with SQLX offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

CMD ["target/release/excretor"]
