![CI](https://github.com/Dinars34/rustgate/actions/workflows/ci.yml/badge.svg)x`
# RustGate

An asynchronous, task-per-connection TCP reverse proxy built in Rust with [Tokio](https://tokio.rs/), designed as a foundation for a fully-featured L4/L7 load balancer.

> **Status:** Active development — core L4 proxy is operational. Load balancing, L7 routing, and health checking are in progress (see [Roadmap](#roadmap)).

---

## Overview

RustGate accepts incoming TCP connections on a configurable frontend port and transparently tunnels traffic to an upstream server using `tokio::io::copy_bidirectional`. Each client connection is handled by an independent async task, allowing the proxy to serve many concurrent connections efficiently on a small number of OS threads.

This project is a learning-driven systems engineering exercise, built while studying async Rust (Tokio, *The Rust Programming Language*). The goal is a production-representative proxy with load balancing, health checking, and observability.

---

## Architecture

```
                          ┌──────────────────────────────────┐
  Client (curl / browser) │           RustGate               │   Upstream Server
  ──────────────────────► │  TcpListener (accept loop)       │ ──────────────────►
                          │    │                             │
                          │    └── tokio::spawn (per conn)  │
                          │         └── copy_bidirectional   │
                          └──────────────────────────────────┘
```

**Current flow:**

1. `TcpListener` binds to the frontend port and enters an accept loop.
2. For each incoming connection, `tokio::spawn` creates an independent async task — the loop is never blocked.
3. Each task opens a new TCP connection to the upstream server and calls `copy_bidirectional`, which tunnels bytes in both directions until either side closes.
4. Connection statistics (bytes transferred) are logged to stdout.

---

## Features

### Implemented
- **Async L4 TCP proxy** — bidirectional byte forwarding via `tokio::io::copy_bidirectional`
- **Task-per-connection model** — `tokio::spawn` ensures concurrent connections never block each other
- **Upstream error handling** — logs and drops connections gracefully when the upstream is unreachable
- **Integration test** — end-to-end test that spins up a mock backend and asserts the proxy correctly forwards traffic
- **CI pipeline** — GitHub Actions runs `cargo fmt`, `cargo clippy`, and `cargo test` on every push

### Roadmap
- [ ] **Round-robin load balancing** across multiple upstream backends (`Arc<RwLock<Vec<Backend>>>`)
- [ ] **Least-connections strategy** — route to the backend with the fewest active connections
- [ ] **`LoadBalancer` trait** — pluggable strategy via trait objects (`dyn LoadBalancer`)
- [ ] **Active health checking** — background task pings each backend every N seconds; marks unhealthy backends as unavailable
- [ ] **Passive health checking** — mark backend unhealthy on connection failure; re-probe after cooldown
- [ ] **L7 routing** — parse HTTP host/path headers to route requests to different backend pools
- [ ] **Graceful shutdown** — drain active connections on `SIGINT`/`SIGTERM` before exit
- [ ] **Structured logging** — replace `println!` with `tracing` for levelled, structured output
- [ ] **Prometheus metrics** — expose request count, error rate, and latency histograms at `/metrics`
- [ ] **File-based configuration** — load upstream addresses and routing rules from a config file (`serde`)
- [ ] **Docker Compose demo** — single `docker compose up` to start RustGate + multiple dummy backends

---

## Getting Started

### Prerequisites

- Rust (stable) — install via [rustup](https://rustup.rs/)
- Cargo (bundled with Rust)

```bash
rustup update stable
```

### Build

```bash
git clone https://github.com/Dinars34/rustgate.git
cd rustgate
cargo build
```

### Run

Start an upstream server on port 3000 (any HTTP server will work):

```bash
# Example: Python one-liner
python3 -m http.server 3000
```

Then start the proxy:

```bash
cargo run
```

RustGate will listen on `127.0.0.1:8080` and forward all traffic to `127.0.0.1:3000`.

```bash
# Verify
curl http://127.0.0.1:8080
```

Expected terminal output:
```
[RustGate Frontline] Async Gateway v0.1.0 online. Listening on 8080...
Connection Success
Connection closed normally. 78 bytes -> server, 154 bytes -> client
```

---

## Running Tests

```bash
cargo test
```

The integration test (`tests/proxy_test.rs`) spins up a mock TCP backend that responds with `HTTP/1.1 200 OK`, starts the proxy, sends a real HTTP request through it, and asserts the response passes through correctly — no mocking of the proxy internals.

---

## Project Structure

```
rustgate/
├── src/
│   ├── main.rs          # Binary entry point — wires up start_proxy()
│   └── lib.rs           # Core proxy logic: start_proxy(), handle_connection()
├── tests/
│   └── proxy_test.rs    # End-to-end integration test
├── notes/
│   └── ch16.md          # Learning notes on concurrency (Ch. 16 TRPL)
├── .github/
│   └── workflows/
│       └── ci.yml       # CI: fmt + clippy + test
└── Cargo.toml
```

---

## Technical Notes

### Why async over threads?

A proxy's workload is almost entirely I/O-bound: it spends most of its time waiting for bytes from the client or upstream — not computing. Spawning one OS thread per connection wastes memory and scheduler overhead at scale. Tokio's async model allows a small thread pool to drive thousands of concurrent connections by suspending tasks at `.await` points and resuming them only when I/O is ready.

### `copy_bidirectional`

`tokio::io::copy_bidirectional` concurrently copies bytes in both directions (client→upstream and upstream→client) within a single task. It returns `(bytes_to_server, bytes_to_client)` when either side signals EOF, making it the idiomatic Tokio primitive for a TCP tunnel.

### Crate separation (`main.rs` / `lib.rs`)

Logic lives in `lib.rs` (`start_proxy`, `handle_connection`) so it can be imported by integration tests in `tests/`. `main.rs` is a thin binary that calls into the library. This mirrors the pattern recommended in *The Rust Programming Language*, Chapter 12.

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (edition 2024) |
| Async runtime | Tokio 1.38 (`features = ["full"]`) |
| CI | GitHub Actions |

---

## Learning Context

RustGate is built alongside a systematic study of *The Rust Programming Language* (Brown University edition). Chapters 16 (concurrency: threads, channels, `Arc<Mutex<T>>`) and 17 (async/await, Tokio) directly informed the core architecture. Planned features map to Ch. 18 (trait objects for the `LoadBalancer` trait) and Ch. 19 (config parsing with serde).

Development notes are tracked in `notes/` as each chapter is applied to the project.

---