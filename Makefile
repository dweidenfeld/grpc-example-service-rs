.PHONY: help

# ------------------------------------------------------
# VARIABLES
# ------------------------------------------------------
DOCKER=docker
COMPOSE=docker-compose

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# ------------------------------------------------------
# GLOBAL
# ------------------------------------------------------
test: # TODO
	$(COMPOSE) run --rm app cargo test
#	$(COMPOSE) run --rm app cargo tarpaulin -v -o Html

test-watch: # TODO
	$(COMPOSE) run --rm app cargo watch -x test

clean: # TODO
	$(COMPOSE) down
	$(COMPOSE) rm -f

build: # TODO
	$(COMPOSE) build

start: # TODO
	$(COMPOSE) up

lint: # TODO
	$(COMPOSE) run --rm app cargo fmt
	$(COMPOSE) run --rm app cargo clippy
