FROM rust:slim AS builder

WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/myapp/target/release/excretor /usr/local/bin/excretor
CMD ["excretor"]