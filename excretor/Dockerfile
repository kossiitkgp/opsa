FROM rust:slim-buster

WORKDIR /usr/src/myapp

# Copy all files
COPY Cargo.toml .
COPY . .

# Build the binary
RUN cargo build --release

CMD ["target/release/excretor"]