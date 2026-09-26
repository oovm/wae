# `@wae/protocol`

Protocol **runtime**: encode client / host messages to JSON strings, construct RPC request/response envelopes. Types
live in `@wae/types`; ID generation uses `@wae/core`.

## Install

```bash
pnpm add @wae/protocol@0.0.0
```

## Message kinds

**Client → Host (`ClientMessage`)**

| `type`    | Meaning                             |
|-----------|-------------------------------------|
| `uiEvent` | UI event                            |
| `rpc`     | RPC request (includes `RpcRequest`) |
| `native`  | Native capability call              |

**Host → Client (`HostMessage`)**

| `type`     | Meaning        |
|------------|----------------|
| `domPatch` | DOM patch list |
| `rpc`      | RPC response   |
| `error`    | `WaeError`     |

## API

```ts
import {
  encodeMessage,
  decodeClientMessage,
  decodeHostMessage,
  createRpcRequest,
  okRpcResponse,
  errRpcResponse,
  hostDomPatches,
} from "@wae/protocol";

const req = createRpcRequest("ping", { n: 1 });
const wire = encodeMessage({ type: "rpc", request: req });
const back = decodeClientMessage(wire);
```

| Function                           | Description                             |
|------------------------------------|-----------------------------------------|
| `encodeMessage`                    | `JSON.stringify`                        |
| `decode*`                          | `JSON.parse` (**no** schema validation) |
| `createRpcRequest`                 | Auto `createRequestId()`                |
| `okRpcResponse` / `errRpcResponse` | Build responses                         |
| `hostDomPatches`                   | Wrap `DomPatch[]`                       |

## Version / handshake / compatibility (current)

| Item                   | 0.0.0                                                                              |
|------------------------|------------------------------------------------------------------------------------|
| Protocol version field | **No** separate version frame                                                      |
| Handshake              | **None**                                                                           |
| Compatibility          | Relies on JSON shape; unknown fields preserved by `JSON.parse` but not in TS types |
| Runtime validation     | **None**; wrong shapes rely on convention                                          |

Extension fields: before handshake exists, do not rely on fields not written in `@wae/types` for production
compatibility.

## Related

- [`@wae/types`](../public-types/readme.md)
- Host: [`host/bridge`](../../crates/wae-bridge/readme.md)
