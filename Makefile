SHELL := /bin/bash
CONTAINER_ID = $(shell docker ps -q --filter "name=tummy" --filter "status=running")

default: build run 

build:
	docker compose build

digest:
	@if [[ -z "$(FILE)" ]]; then \
        echo "ERROR: No file path provided. Please specify the file path using 'make digest FILE=/path-to-file'"; \
        exit 1; \
    fi;
	@if [[ -n "$(CONTAINER_ID)" ]]; then \
		$(MAKE) run-digester; \
	else \
		docker compose up tummy -d; \
		$(MAKE) run-digester; \
		docker compose down; \
	fi; \
	echo "Digestion complete.";

.PHONY: run-digester
run-digester:
	ZIPFILE_PATH=$(FILE) docker compose -f docker-compose-digester.yml up --build --abort-on-container-exit; \
	ZIPFILE_PATH=$(FILE) docker compose -f docker-compose-digester.yml down; \

excrete:
	docker compose --profile excrete up -d

run:
	docker compose --profile excrete --profile digest up -d

stop:
	docker compose stop
