DATABASE_VOLUME := food

ifeq (, $(shell which docker-compose))
    DOCKER_COMPOSE=docker compose
else
    DOCKER_COMPOSE=docker-compose
endif

# List of all targets which are meant to be run by the user (should have a description)
ALL_TARGETS := $(shell sed -n 's/^## //p' ${MAKEFILE_LIST} | awk -F ':' '{print $$1}')

PROJECT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
TUMMY_CONTAINER_ID = $(shell docker ps -q --filter "name=tummy" --filter "status=running")

default: build run

.PHONY: ALL_TARGETS

%:
	@if [[ -z "$(filter $(MAKECMDGOALS), $(ALL_TARGETS))" ]]; then \
		echo "Target '$(MAKECMDGOALS)' not found."; \
		echo ""; \
		$(MAKE) --no-print-directory help; \
	fi


## help: Show this help message
help:
	@echo "Usage: make [target]"
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
	@echo ""
	@echo "Running 'make' without a target is equivalent to running 'make build run'."

## build: Build the excreter and tummy docker images
build:
	$(DOCKER_COMPOSE) build

## run: Run the excreter and tummy docker containers
run:
	$(DOCKER_COMPOSE) up -d

## stop: Stop the excreter and tummy docker containers
stop:
	$(DOCKER_COMPOSE) stop
	$(DOCKER_COMPOSE) down

## digest: Run the digester container
digest:
	@if [[ -z "$(FILE)" ]]; then \
        echo "ERROR: No file path provided. Please specify the file path using 'make digest FILE=/path-to-file'"; \
        exit 1; \
    fi;
	@if [[ -n "$(TUMMY_CONTAINER_ID)" ]]; then \
		$(MAKE) run-digester; \
	else \
		$(DOCKER_COMPOSE) up tummy -d; \
		$(MAKE) run-digester; \
		$(DOCKER_COMPOSE) down; \
	fi; \
	echo "Digestion complete.";

run-digester:
	ZIPFILE_PATH=$(FILE) $(DOCKER_COMPOSE) -f docker-compose-digester.yml up --build --abort-on-container-exit; \
	ZIPFILE_PATH=$(FILE) $(DOCKER_COMPOSE) -f docker-compose-digester.yml down; \

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