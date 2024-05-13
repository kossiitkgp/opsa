SHELL := /bin/bash

DATABASE_VOLUME := food

PROJECT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
CONTAINER_ID = $(shell docker ps -q --filter "name=tummy" --filter "status=running")

default: build run 

build:
	docker compose build

run:
	docker compose up -d

stop:
	docker compose stop
	docker compose down


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

.PHONY: check_clean
check_clean:
	@echo "This will remove the database volume. This action is irreversible."
	@echo -n "Are you sure? [y/N] " && read ans && [ $${ans:-N} = y ]

.PHONY: clean
clean: check_clean
	docker volume rm $(notdir $(PROJECT_DIR))_$(DATABASE_VOLUME)