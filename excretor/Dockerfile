FROM rust:slim

WORKDIR /usr/src/myapp
COPY . .

RUN cargo fetch
RUN cargo build --release
RUN cp target/release/excretor /

CMD ["/excretor"]
