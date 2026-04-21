ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
DOCKER_COMPOSE := $(ROOT)/docker-compose.yml

setup-rustup:
	# rust-lang.rust-analyzer needs this
	curl \
	--proto '=https' \
	--tlsv1.2 -sSf https://sh.rustup.rs | sh

build:
	docker compose -f $(DOCKER_COMPOSE) build

run:
	docker compose -f $(DOCKER_COMPOSE) run --rm alpine
	docker compose -f $(DOCKER_COMPOSE) run --rm trixie
	docker compose -f $(DOCKER_COMPOSE) run --rm fedora

stop:
	docker compose -f $(DOCKER_COMPOSE) down

logs:
	docker compose -f $(DOCKER_COMPOSE) logs -f

cargo-build:
	cargo clean && cargo build

cargo-run-server:
	cargo run --release --bin server

cargo-run-server-dev:
	cargo run --bin server

cargo-run-benchmark:
	cargo run --release --bin client

cargo-run-benchmark-dev:
	cargo run --bin client
