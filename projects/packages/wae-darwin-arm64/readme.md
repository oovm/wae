# `@wae/wae-darwin-arm64`

Apple Silicon host entry. Unlike ios-arm64: this package is a desktop shell, not an App Store iOS bundle.

## For application users

| Item                       | Details                                                                                                    |
|----------------------------|------------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-darwin-arm64` @ `0.0.0`                                                                          |
| Platform id                | `darwin-arm64`                                                                                             |
| Who needs it               | macOS Apple Silicon desktop WebView                                                                        |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-darwin-arm64@0.0.0` |
| os/cpu                     | os: darwin · cpu: arm64                                                                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                     |
| Runtime requirements       | macOS arm64; native binary not bundled.                                                                    |
| Not supported / incomplete | No .app.                                                                                                   |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                         |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-darwin-arm64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Build on Apple Silicon Mac.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
