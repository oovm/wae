# `wae-bridge`

Rust crate: **bridge** between WebView / WASM / native host and frontend. Works with `@wae/client`, `@wae/protocol`, and
`@wae/wae-*`.

**Not** an npm package; `publish = false`.

## Call direction

```text
ClientMessage (frontend)
  → MessageTransport::send_json / host receives
  → WebViewRuntime::handle
  → Option<HostMessage> (back to frontend)
```

| API                      | Meaning                                                    |
|--------------------------|------------------------------------------------------------|
| `MessageTransport`       | `send_json(&str) -> Result<()>`                            |
| `WebViewRuntime::handle` | Handle `ClientMessage`; **returns `None` always in 0.0.0** |
| `WebViewBridge`          | Lifecycle placeholder                                      |

Sync / async: current `handle` is sync; real WebView callbacks are often async—to be decided when wired.

## Serialization and errors

- Message types from crate `wae-types` (schema)
- JSON payloads target alignment with TS `@wae/protocol`; shape changes need both sides
- Permissions: native capabilities should be authorized on host; frontend `native` messages must not be trusted by
  default

## Testing without a full desktop shell

```bash
cargo test -p wae-bridge
cargo check -p wae-bridge
```

Inject sample `ClientMessage` into `WebViewRuntime::handle` and assert `HostMessage` (once implemented).

## Related

- [`@wae/protocol`](../../packages/protocol/readme.md)
- Platform packages: [`packages`](../../packages/readme.md)
