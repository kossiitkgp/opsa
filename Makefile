default: build run 

build:
	docker compose build

digest:
	@if [ -z "$(FILE)" ]; then \
        echo "ERROR: No file path provided. Please specify the file path using 'make digest FILE=/path-to-file'"; \
        exit 1; \
    fi
	ZIPFILE_PATH=$(FILE) docker compose --profile digest up --build --abort-on-container-exit
	ZIPFILE_PATH=$(FILE) docker compose --profile digest down

excrete:
	docker compose --profile excrete up -d

run:
	docker compose --profile excrete --profile digest up -d

stop:
	docker compose stop
