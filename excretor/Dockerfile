FROM rust:slim

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["excretor"]
