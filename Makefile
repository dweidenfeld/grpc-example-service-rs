.PHONY: help

# ------------------------------------------------------
# VARIABLES
# ------------------------------------------------------
DOCKER=docker
COMPOSE=docker-compose -f docker-compose.yml

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# ------------------------------------------------------
# COMMANDS
# ------------------------------------------------------
clean: ## Stop the dev environment and clean old docker containers
	@$(COMPOSE) down

install: ## Install dependencies on host system (required for start/test/test-live)
	@$(COMPOSE) run --rm service cargo build

format: ## Reformat the code on style errors
	@$(COMPOSE) run --rm service cargo fmt

build: ## Rebuild docker images (e.g. on dependency changes)
	@$(COMPOSE) build

start: ## Start the dev environment
	@$(COMPOSE) up

test: ## Execute tests
	@$(COMPOSE) run --rm service cargo test
#
test-live: ## Start live testing environment
	@$(COMPOSE) run --rm service cargo watch -x test

