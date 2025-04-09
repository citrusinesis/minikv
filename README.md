# 🧠 MiniKV

A minimal, extensible, and type-safe key-value store built in Rust.  
Designed for both server-based and embedded usage, with protocol-agnostic communication, middleware-driven processing, and pluggable storage backends.

## Overview

### ✨ Features

MiniKV is organized into three conceptual layers:

- **Client**  
  Issues commands through a CLI or via embedded usage in applications.

- **Server (Optional)**  
  Supports multiple protocol frontends (e.g. TCP, HTTP, gRPC) and composable middleware layers using `tower::Service`.

- **Core**  
  Provides a clean protocol and storage abstraction. Can be used directly as an SDK within Rust applications without requiring a network server.

### ✅ Highlights

- ✅ **Well-abstracted architecture** supporting multiple protocols and backends
- ✅ Decoupled command model using `Command` & `Response` enums
- ✅ Protocol-agnostic design with potential support for:
  - 🧩 JSON over TCP
  - 🌐 HTTP (REST, WebSocket)
  - 🔗 gRPC or custom binary protocols
- ✅ Operates **with or without a server** – usable as a library
- ✅ Pluggable backend system:
  - 🧠 In-memory storage
  - 💾 File-based or persistent databases
  - 🌉 Remote or inter-system integrations (e.g. Redis, cluster)
- ✅ Middleware-enabled server powered by `tower::Service`
- ✅ Type-safe, versionable protocol defined via `serde`

### 🧩 Architecture Overview

MiniKV is structured around clean separation between core logic, frontend protocol, and backend storage.

```
[ Client ]
    |
    v
+--------------------------+
|        Server            |
| +----------------------+ |
| |  HTTP / gRPC / TCP   | |  ← protocol frontends
| +----------+-----------+ |
|            v             |
|      Middleware Layer    |
|            v             |
|     Protocol Layer       |
+------------+-------------+
             |
             v
+--------------------------+
|          Core            |
|                          |
|  +--------------------+  |
|  | Storage Abstraction|  |  ← InMemory / File / etc.
|  +--------------------+  |
|           ^              |
|           |              |
|    Used as SDK           |
+--------------------------+
```

### 📦 Designed for Extensibility

MiniKV is built from the ground up to be **modular, protocol-agnostic, and easily embeddable**.

- **Pluggable Protocol Layer**: Decoupled from transport, allowing support for TCP, HTTP, gRPC, or even in-process execution.
- **Flexible Frontends**: Easily integrate MiniKV into CLIs, REST APIs, background workers, or interactive UIs.
- **Swappable Backends**: Implement the `Storage` or `SharedStorage` trait to use anything from in-memory stores to file-based databases or external systems.

> While the current implementation may include only one frontend or backend, the architecture is intentionally designed to make adding new protocols or storage engines straightforward.

## ⚙️ Usage

### Start the server

```bash
cargo run -p minikv-server
```

### Run CLI (single command)

```bash
cargo run -p minikv-cli -- set foo bar
cargo run -p minikv-cli -- get foo
cargo run -p minikv-cli -- del foo
```

### Run CLI REPL mode (default)

```bash
cargo run -p minikv-cli
```

You’ll see:

```
MiniKV REPL started. Type `exit` or `quit` to leave.
minikv> set foo hello
[OK] OK
minikv> get foo
[OK] hello
minikv> del foo
[OK] Deleted
minikv> get foo
[ERROR] KeyNotFound
```


## 📡 Request / Response Payload

Currently MiniKV uses a custom JSON-based TCP protocol.

### 📤 Request (`Command`)

```json
{ "cmd": "set", "key": "foo", "value": "bar" }
{ "cmd": "get", "key": "foo" }
{ "cmd": "del", "key": "foo" }
```

### 📥 Response (`Response`)

```json
{ "status": "ok", "value": "bar" }
{ "status": "error", "message": "KeyNotFound" }
```


## 🛠️ Dev & Debug

Enable full logs:

```bash
RUST_LOG=info cargo run -p minikv-server
```

Example log (with `tracing`):

```
2025-04-09T06:39:03.396748Z  INFO Starting MiniKV server on 127.0.0.1:4000
2025-04-09T06:39:03.396838Z  INFO Server started addr=127.0.0.1:4000
2025-04-09T06:39:21.451960Z  INFO Handled command req=Set { key: "foo", value: "hello" } val=OK took=36.31µs
```
