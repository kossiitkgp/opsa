FROM golang:1.22.3-alpine

WORKDIR /app

RUN apk add build-base

COPY . /app

RUN go mod download

RUN go build -o app .

EXPOSE 8080

CMD ./app