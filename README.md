# Distro Bench

A Rust benchmarking tool that compares runtime performance across Linux distributions.
Runs the same server/client workload in isolated containers and records performance results.

## Metrics

- Total requests processed
- Execution time
- Latency (p50 / p95 / p99)
- Success rate
- OS / distro differences

## Project Layout

```
src/                  Rust server + client
scripts/              Runtime scripts
docker-compose.yml    Multi-distro setup
Dockerfile.*          OCIs under test
Makefile              Commands
.cargo-home/          Dependency cache (local)
.cargo-target/        Build output cache (local)
results/              Benchmark output files
```

## Quick Start

### Setup cache

- `.cargo-home` → dependencies
- `.cargo-target` → build artifacts

```bash
make cargo-init-cache
```

## Local Run (Rust required)

### Build debug

```bash
make cargo-build
```

### Build release (recommended)

```bash
make cargo-build-release
```

### Run server

```bash
make cargo-run-server
```

### Run benchmark

```bash
make cargo-run-benchmark
```

## Docker Benchmark (Alpine / Debian / Fedora)

### Build

```bash
make build
```

### Run

- server starts
- client runs workload
- results saved to `./results`

```bash
make run
```

### Stop

```bash
make stop
```

### View logs

```bash
make logs
```

## Clean

```bash
make cargo-clean
```

Removes:

- `.cargo-home`
- `.cargo-target`

## Output

All benchmark results go to:

```
./results
```

Each run contains:

- execution time
- request count
- latency (p50 / p95 / p99)
- success rate
- OS metadata

## Execution Flow

```
Makefile
   ↓
Docker Compose
   ↓
Alpine / Debian / Fedora
   ↓
run.sh
   ↓
server + client
   ↓
results
```

## Typical Workflow

```bash
make cargo-init-cache
make cargo-build-release
make build
make run
```

## Key Design

- Makefile central control
- Docker ensures identical workloads
- Results compare OS performance only
