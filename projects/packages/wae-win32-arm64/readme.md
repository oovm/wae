# `@wae/wae-win32-arm64`

Same role as win32-x64, architecture arm64. Do not force-install on x64 machines as runtime.

## For application users

| Item                       | Details                                                                                                   |
|----------------------------|-----------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-win32-arm64` @ `0.0.0`                                                                          |
| Platform id                | `win32-arm64`                                                                                             |
| Who needs it               | Windows ARM64 desktop WebView                                                                             |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-win32-arm64@0.0.0` |
| os/cpu                     | os: win32 · cpu: arm64                                                                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                    |
| Runtime requirements       | Windows ARM64; native binary not bundled.                                                                 |
| Not supported / incomplete | Same as other desktop packages: no prebuilt host.                                                         |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                        |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-win32-arm64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Requires win32-arm64 or cross-compile environment (no script provided).

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
