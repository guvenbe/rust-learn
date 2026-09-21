# Monorepo Demo — User Management System

This repository contains a **demo application** that simulates a small **User Management System**.
It is built as a **Cargo workspace monorepo** to illustrate the strategies described in the chapter *“Monorepo Strategies with Cargo”*.

## What does the project do?

The project models a very simple domain: **users**.

* A **user** has an `id` (number) and a `name` (string).
* New users are created from a `NewUser` payload that must pass **validation rules**:

  * The name must have at least 3 characters.
  * Control characters are not allowed.

Around this shared model, the system is split into four crates:

1. **`libraries/data-model` (library):**
   Defines `User`, `NewUser`, validation logic, and domain errors.
   Other crates depend on this library.

2. **`services/api-server` (HTTP service):**
   Exposes an HTTP API with three endpoints:

   * `GET /health` → shows service status and user count.
   * `POST /users` → creates a new user (validates input).
   * `GET /users/:id` → fetches a user by ID.
     Users are stored in memory.

3. **`services/worker` (background service):**
   Simulates processing a queue of `NewUser` events and converting them into `User` records.
   Demonstrates background tasks reusing the shared library.

4. **`tools/migrate` (CLI tool):**
   Mimics a database migration utility with commands like `status`, `apply`, and `revert`.
   It also has a `demo-user` command that validates a user name via the shared library.

Together, these crates form a **mini system**: one shared domain library, two services, and a tool — exactly the kind of scenario where monorepos shine.

---

## Building and Testing

At the root of the repository:

```bash
cargo build
cargo test
```

* **Builds all crates** in one shot, sharing the same `Cargo.lock` and `target/`.
* **Runs all tests** across the workspace, including unit and integration tests from `data-model`.

---

## Running Each Component

### 1. Library example (`data-model`)

Demonstrates how to run examples from a library crate:

```bash
cargo run -p data-model --example demo
```

You’ll see a sample `User` printed to the console.

---

### 2. API server (`api-server`)

Start the HTTP server:

```bash
cargo run -p api-server
```

In another terminal, test it with `curl`:

```bash
curl -s http://127.0.0.1:3000/health
curl -s -X POST http://127.0.0.1:3000/users -H 'content-type: application/json' -d '{"name":"Alice"}'
curl -s http://127.0.0.1:3000/users/1
```

This shows how a service crate reuses types and validation from the shared library.

---

### 3. Worker service (`worker`)

Run the worker:

```bash
cargo run -p worker
```

You’ll see logs of “events” being processed into users.
This simulates a background job processor reusing the same domain code.

---

### 4. CLI tool (`migrate`)

Run the CLI tool with different subcommands:

```bash
cargo run -p migrate -- status
cargo run -p migrate -- demo-user "Eve"
```

This demonstrates how internal tools in a workspace can share domain logic.

---

## How this demonstrates monorepo strategies

* **Workspace structure (nested):**
  `libraries/*`, `services/*`, `tools/*` all live under one root, defined in `Cargo.toml`.

* **Shared dependencies:**
  Versions of `serde`, `tokio`, `anyhow`, etc., are declared once in `[workspace.dependencies]`.
  All crates reuse them with `workspace = true`.

* **Selective build/test:**
  You can build or test the entire monorepo, or just one crate using `-p`.

* **Cross-crate reuse:**
  `api-server`, `worker`, and `migrate` all import `data-model` to ensure consistent validation.

* **Examples and integration tests:**
  `data-model` includes both, demonstrating how to ship runnable code and enforce compatibility.

* **Publish hygiene:**
  Internal crates (`api-server`, `worker`, `migrate`) have `publish = false`, preventing accidental publishing.

* **No external tools:**
  Everything is done with standard Cargo features — no custom scripts or third-party monorepo managers.

---

## Summary

This repository is not production software.
It is a **teaching tool** that shows how to:

* Model a simple domain (`User`) once, in a shared library.
* Reuse that domain across services and tools.
* Structure and operate a monorepo efficiently using Cargo.

With this small but realistic system, you can see how Rust’s workspaces make monorepo strategies simple, consistent, and scalable.
