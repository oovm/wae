# backend/rust/native-service

## What this example demonstrates

Rust native service placeholder for local capabilities (files, devices), related to WebView host but not TS server.

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
Frontend / other process
  → IPC or local protocol
  → Rust native service
  → system calls
```

## Exercises

- Contrast TS examples like `native/filesystem` (page side) with this directory (service process intent).
- Testable bridge lives in `wae-bridge` / host crates.

## Differences from production apps

Production Rust services are separate deploy units; WAE host is local shell. When both are “backend”, label which.

Dependencies: none in this directory. Not in examples pnpm filter.
