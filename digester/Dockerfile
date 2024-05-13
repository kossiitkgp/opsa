FROM golang:1.22.3-alpine

WORKDIR /app

RUN apk add build-base

COPY --from=ghcr.io/ufoscout/docker-compose-wait:latest /wait /wait

COPY . /app

RUN go mod download

RUN go build -o app ./main.go

EXPOSE 8080

CMD /wait && ./app