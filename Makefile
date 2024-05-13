SHELL := /bin/bash
DATABASE_VOLUME := food

PROJECT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
CONTAINER_ID = $(shell docker ps -q --filter "name=tummy" --filter "status=running")

default: build run

.PHONY: help build run stop digest run-digester check_clean clean

## help: Show this help message
help:
	@echo "Usage: make [target]"
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'

## build: Build the excreter and tummy docker images
build:
	docker compose build

## run: Run the excreter and tummy docker containers
run:
	docker compose up -d

## stop: Stop the excreter and tummy docker containers
stop:
	docker compose stop
	docker compose down

## digest: Run the digester container
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

run-digester:
	ZIPFILE_PATH=$(FILE) docker compose -f docker-compose-digester.yml up --build --abort-on-container-exit; \
	ZIPFILE_PATH=$(FILE) docker compose -f docker-compose-digester.yml down; \

check_clean:
	@echo "This will remove the database volume. This action is irreversible."
	@echo -n "Are you sure you want to proceed? [y/N] " && read ans; \
    if [ $${ans:-N} != y ] && [ $${ans:-N} != Y ]; then \
        echo "Operation canceled."; \
        exit 1; \
    fi

## clean: Remove the database volume
clean: check_clean
	@docker volume rm $(notdir $(PROJECT_DIR))_$(DATABASE_VOLUME)
	@echo "Database volume removed."