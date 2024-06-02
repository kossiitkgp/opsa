MAKEQ := $(MAKE) --no-print-directory

ifeq (, $(shell which docker-compose))
    DOCKER_COMPOSE=docker compose
else
    DOCKER_COMPOSE=docker-compose
endif

CURRENT_MAKEFILE := $(lastword $(MAKEFILE_LIST))
PROJECT_DIR := $(shell dirname $(realpath $(CURRENT_MAKEFILE)))

ENVS := $(shell grep -v '^#' .env)
EXCRETOR_DEV_ENVS := $(ENVS) RUST_BACKTRACE=1

DATABASE_VOLUME := food

default: build run

## help: Show this help message
.PHONY: help
help:
	@echo "Usage: make [target]"
	@sed -n 's/^##//p' $(CURRENT_MAKEFILE) | column -t -s ':' |  sed -e 's/^/ /'
	@echo ""
	@echo "Running 'make' without a target is equivalent to running 'make build run'."

## dev: Run the excretor in development mode
.PHONY: dev
dev:
	@echo "Starting tummy-dev with exposed port"
	@$(DOCKER_COMPOSE) up tummy-dev -d --wait
	@echo ""
	@echo "Running sqlx prepare..."
	@cd excretor && cargo sqlx prepare && cd ..
	@echo ""
	@echo "Starting excretor in development mode."
	@bash -c "trap 'echo "";$(MAKEQ) dev-stop; exit 0' SIGINT SIGTERM ERR; $(EXCRETOR_DEV_ENVS) cargo watch -C '$(PROJECT_DIR)/excretor/' -c -x run --ignore '*.css';"
# In case the excretor gracefully shuts down
	@$(MAKEQ) dev-stop

## dev-stop: Stop the tummy-dev docker container
.PHONY: dev-stop
dev-stop:
	@echo ""
	@echo "Stopping tummy-dev docker container..."
	@$(DOCKER_COMPOSE) stop tummy-dev
	@$(DOCKER_COMPOSE) down tummy-dev

## build: Build the excretor and tummy docker images
.PHONY: build
build:
	@echo "Building excretor and tummy docker images..."
	@$(DOCKER_COMPOSE) build excretor tummy

## run: Run the excretor and tummy docker containers
.PHONY: run
run:
	@echo "Running excretor and tummy docker containers..."
	@$(DOCKER_COMPOSE) up excretor tummy -d

## stop: Stop the excretor and tummy docker containers
.PHONY: stop
stop:
	@echo "Stopping excretor and tummy docker containers..."
	@$(DOCKER_COMPOSE) stop excretor tummy
	@$(DOCKER_COMPOSE) down excretor tummy

## digest: Run the digester container
.PHONY: digest
digest:
ifeq (, $(FILE))
	@echo "ERROR: No file path provided. Please specify the file path using 'make digest FILE=/path-to-file'"
	@exit 1;
endif
	@echo "Starting tummy-dev with exposed port"
	@$(DOCKER_COMPOSE) up tummy-dev -d --wait;
	@echo ""
	@echo "Starting digester..."
	@bash -c "trap 'echo ""; popd > /dev/null && $(MAKEQ) dev-stop; exit 0' SIGINT SIGTERM ERR; pushd $(PROJECT_DIR)/digester > /dev/null && go mod download && ZIPFILE_PATH='$(FILE)' $(ENVS) go run .;"
# In case the digester gracefully shuts down
	@$(MAKEQ) dev-stop

.PHONY: check_clean
check_clean:
	@echo "This will remove the database volume. This action is irreversible."
	@echo -n "Are you sure you want to proceed? [y/N] " && read ans; \
    if [ $${ans:-N} != y ] && [ $${ans:-N} != Y ]; then \
        echo "Operation canceled."; \
        exit 1; \
    fi

## clean: Remove the database volume
.PHONY: clean
clean: check_clean
	@docker volume rm $(notdir $(PROJECT_DIR))_$(DATABASE_VOLUME)
	@echo "Database volume removed."

%:
ifneq (, $(MAKECMDGOALS))
	@echo "Target '$(MAKECMDGOALS)' not found."
	@echo ""
	@$(MAKEQ) help
endif
