# `@wae/core`

Small **shared runtime** for client and server: generate IDs, recognize `WaeError`, extract error messages. No HTTP app,
no protocol JSON, no DOM/UI.

## Install

```bash
pnpm add @wae/core@0.0.0
```

## API

| Function                | Behavior                                  |
|-------------------------|-------------------------------------------|
| `createRequestId()`     | Returns `req_<time>_<rand>`               |
| `createNodeId()`        | Returns `node_<rand>`                     |
| `isWaeError(value)`     | Structural check (has `code` + `message`) |
| `toErrorMessage(error)` | `WaeError` / `Error` / other → string     |

## Message semantics (this package)

| Question                 | Answer (0.0.0)                                                                              |
|--------------------------|---------------------------------------------------------------------------------------------|
| Does request have an ID? | Caller or `@wae/protocol.createRpcRequest` generates; this package only provides generators |
| Cancellation             | **Does not** implement Abort orchestration; pass standard `AbortSignal` yourself            |
| Delivery guarantee       | **None**; this package does not transport                                                   |
| Ordering / concurrency   | **No** queue semantics                                                                      |

Transport and at-least-once delivery belong to transport / host, not core.

## Related

- Types: [`@wae/types`](../public-types/readme.md)
- Encode/decode: [`@wae/protocol`](../protocol/readme.md)
