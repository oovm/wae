# `@wae/wae-linux-arm64`

Linux arm64 host entry (e.g. some SBC / ARM server desktops).

## For application users

| Item                       | Details                                                                                                   |
|----------------------------|-----------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-linux-arm64` @ `0.0.0`                                                                          |
| Platform id                | `linux-arm64`                                                                                             |
| Who needs it               | Linux arm64 desktop WebView                                                                               |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-linux-arm64@0.0.0` |
| os/cpu                     | os: linux · cpu: arm64                                                                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                    |
| Runtime requirements       | Linux arm64; binary not bundled.                                                                          |
| Not supported / incomplete | Same as linux-x64.                                                                                        |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                        |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-linux-arm64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Build on linux-arm64.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
