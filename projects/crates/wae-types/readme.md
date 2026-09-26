# schema (crate `wae-types`)

Rust-side communication **IDL / type source of truth**. Target: codegen to `@wae/types` (and future bindings). **Not**
an npm package.

## Who should read this

- People changing cross-language message shapes
- People implementing `wae generate types`
- People referencing the same `ClientMessage` / `HostMessage` in Rust host

## Definition and generation (intent)

```text
schema (Rust)
  → wae generate types (CLI, skeleton in 0.0.0)
  → @wae/types (TypeScript)
```

`@wae/types` in the repo today is hand-aligned; changing schema **does not** auto-update npm types until codegen is
wired.

## Change impact

| Change                | Impact                                          |
|-----------------------|-------------------------------------------------|
| Add optional field    | Old clients may ignore; need default convention |
| Remove / rename field | Breaks TS and Rust                              |
| Change `ErrorCode`    | All error handling branches affected            |

Runtime validation: TS `protocol` only does `JSON.parse` today; strict validation belongs in codegen output or a
separate layer if needed.

## Development

```bash
cargo check -p wae-types
```

Unrelated to npm publish; app users usually depend on `@wae/types` / `@wae/protocol` only.
