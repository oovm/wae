# backend/rust/http

## What this example demonstrates

Rust HTTP service placeholder directory: “standalone Rust HTTP service” is not the same layer as WAE TS `@wae/server`.

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

HTTP

  → Rust service (not @wae/server)

  → Response

```

## Exercises

- Read [`projects/crates`](../../../../crates/readme.md) and [`@wae/server`](../../../../packages/server/readme.md);
  write clear responsibility boundary.

- `cargo check -p wae-bridge` familiarizes Rust side—that is host, not this directory’s service.

## Differences from production apps

Production Rust services are separate deploy units; WAE host is local shell. When both are “backend”, label which.

Dependencies: none in this directory. Not in examples pnpm filter.

