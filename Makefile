ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
DOCKER_COMPOSE := $(ROOT)/docker-compose.yml

setup_rustup:
	# rust-lang.rust-analyzer need this
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

build-images:
	docker compose -f $(DOCKER_COMPOSE) build
run-images:
	docker compose -f $(DOCKER_COMPOSE) run --rm alpine
	docker compose -f $(DOCKER_COMPOSE) run --rm trixie
	docker compose -f $(DOCKER_COMPOSE) run --rm fedora
stop-images:
	docker compose -f $(DOCKER_COMPOSE) down
logs-images:
	docker compose -f $(DOCKER_COMPOSE) logs -f

cargo_build:
	cargo clean && cargo build
cargo_run_server_dev:
	cargo run --bin server
cargo_run_server:
	cargo run --release --bin server
cargo_run_benchmark_dev:
	cargo run --bin client
cargo_run_benchmark:
	cargo run --release --bin client
