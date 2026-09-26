# backend/rust/wasm-service

## What this example demonstrates

Rust compiled to Wasm service placeholder: Wasm service differs from “`@wae/client` in a page”.

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
Caller
  → Wasm export / WASI or host embed
  → Rust wasm-service logic
```

## Exercises

- Contrast `frontend/wasm`: that is frontend opt-in; this is service-side Wasm.
- Read platform Wasm package docs; do not assume this directory has wasm-bindgen project.

## Differences from production apps

Production Rust services are separate deploy units; WAE host is local shell. When both are “backend”, label which.

Dependencies: none in this directory. Not in examples pnpm filter.
