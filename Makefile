ROOT 		     := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
DOCKER_COMPOSE   := $(ROOT)/docker-compose.yml
CARGO_HOME       := $(ROOT)/.cargo-home             # Package cache
CARGO_TARGET_DIR := $(ROOT)/.cargo-target           # Build binaries home

setup-rustup:
	# rust-lang.rust-analyzer needs this
	curl \
	--proto '=https' \
	--tlsv1.2 -sSf https://sh.rustup.rs | sh

build:
	docker compose -f $(DOCKER_COMPOSE) build --progress=plain

run:
	docker compose -f $(DOCKER_COMPOSE) run --rm alpine
	docker compose -f $(DOCKER_COMPOSE) run --rm trixie
	docker compose -f $(DOCKER_COMPOSE) run --rm fedora

stop:
	docker compose -f $(DOCKER_COMPOSE) down

logs:
	docker compose -f $(DOCKER_COMPOSE) logs -f

cargo-init-cache:
	mkdir -p $(CARGO_HOME) $(CARGO_TARGET_DIR)

cargo-clean:
	rm -rf $(CARGO_TARGET_DIR) $(CARGO_HOME)

cargo-build: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo build

cargo-build-release: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo build --release

cargo-run-server: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo run --release --bin server

cargo-run-server-dev: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo run --bin server

cargo-run-benchmark: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo run --release --bin client

cargo-run-benchmark-dev: cargo-init-cache
	CARGO_HOME=$(CARGO_HOME) \
	CARGO_TARGET_DIR=$(CARGO_TARGET_DIR) \
	cargo run --bin client
