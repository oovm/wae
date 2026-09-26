# `@wae/wae-unknown-wasm32`

First-class Wasm client runtime entry (minimal host assumptions). Default browser path does not auto-enable this
package.

## For application users

| Item                       | Details                                                                                                      |
|----------------------------|--------------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-unknown-wasm32` @ `0.0.0`                                                                          |
| Platform id                | `unknown-wasm32`                                                                                             |
| Who needs it               | Explicit Wasm client                                                                                         |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-unknown-wasm32@0.0.0` |
| os/cpu                     | No os/cpu restriction; must run in a Wasm-capable host                                                       |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                       |
| Runtime requirements       | wasm32 target; concrete `.wasm` artifacts not bundled in this npm package yet.                               |
| Not supported / incomplete | No prebuilt wasm-bindgen glue or `.wasm` files.                                                              |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                           |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-unknown-wasm32";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

`pnpm --filter @wae/wae-unknown-wasm32 run build`; wire Rust wasm build later.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
