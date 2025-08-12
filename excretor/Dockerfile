FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/myapp

# Copy all files
COPY excretor/Cargo.toml .
COPY excretor/ .
# Copy migrations directory
COPY migrations ./migrations

# Build the binary
RUN cargo build --release

CMD ["target/release/excretor"]