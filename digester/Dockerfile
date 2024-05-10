FROM golang:1.22.3-alpine

WORKDIR /app

COPY . /app

RUN apk add build-base

RUN go build -o app ./main.go

EXPOSE 8080

CMD ["./app"]