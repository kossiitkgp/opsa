FROM rust:slim

WORKDIR /usr/src/myapp

# Create a dummy source file and Cargo.toml used to compile deps first
COPY Cargo.toml .
RUN echo "fn main() {}" > ./dummy.rs

# Replace source file path in Cargo.toml to dummy
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
# Compile deps
RUN cargo build --release

# Change the source file path again
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
# Copy the rest
COPY . .
# Build the actual binary
RUN cargo build --release

CMD ["target/release/excretor"]
