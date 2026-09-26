# `@wae/types`

Cross-package **TypeScript types**. No runtime, no side effects; do not use this package as a serialization library.

Formal types are expected from `wae generate types` against `projects/crates/wae-types` (crate `wae-types`); **the CLI
generate command is still a skeleton in 0.0.0**—types in the repo today are hand-aligned drafts.

## Install

```bash
pnpm add @wae/types@0.0.0
```

## Stable shared surface (current)

| Type                                             | Purpose                              |
|--------------------------------------------------|--------------------------------------|
| `NodeId` / `RequestId` / `RouteId` / `SessionId` | Identifier string aliases            |
| `ErrorCode` / `WaeError`                         | Structured errors                    |
| `DomPatch`                                       | Host → frontend DOM patch operations |
| `UiEvent`                                        | Frontend → host UI events            |
| `RpcRequest` / `RpcResponse`                     | RPC envelopes                        |
| `HostMessage`                                    | Host downstream message union        |

## Safe for wire transport?

| Type                                                               | Network / persistence                                                    |
|--------------------------------------------------------------------|--------------------------------------------------------------------------|
| `WaeError`, `RpcRequest`, `RpcResponse`, `HostMessage`, `DomPatch` | Designed JSON-serializable; use `@wae/protocol` for actual encode/decode |
| Aliases like `NodeId`                                              | Just `string`; semantic constraints at app layer                         |

Domain models used only in-app **should not** go in this package; only cross client / server / host contracts belong
here.

## Versioning

Field sets ship with `0.0.0` placeholder release; breaking changes go through protocol and codegen. On upgrade, check
whether `WaeError.code` and `HostMessage.type` remain distinguishable.
