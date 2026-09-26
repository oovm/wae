# backend/rust/websocket

## What this example demonstrates

Rust WebSocket service placeholder: docs note WS can live in Rust, but this directory has no project.

## Prerequisites

This directory currently **only has README**—no `Cargo.toml` / source; **cannot** `cargo run`, no npm `package.json`.

## Startup commands

No startup commands in this directory. Related host crates you can check:

```bash
cargo check -p wae-bridge
```

## Access URL

None.

## Key files

- This README only (project TBD)

## Request / event path (target semantics)

```text
WS
  → Rust service
  → frames / push
```

## Exercises

- Compare `backend/typescript/websocket`: different language and deploy unit, same “backend without WAE frontend”
  semantics.
- Do not copy a nonexistent `Cargo.toml` from this directory.

## Differences from production apps

Production Rust services are separate deploy units; WAE host is local shell. When both are “backend”, label which.

Dependencies: none in this directory. Not in examples pnpm filter.
