FROM golang:1.23 AS builder

WORKDIR /app
COPY . .
RUN go mod download && \
    go build -o app .

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/app .
EXPOSE 8080
CMD ["./app"]