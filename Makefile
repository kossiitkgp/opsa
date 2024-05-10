default: build run 

build:
	docker compose build

digest:
	docker compose --profile digest up

excrete:
	docker compose --profile excrete up -d

run:
	docker compose --profile excrete --profile digest up -d

stop:
	docker compose stop
