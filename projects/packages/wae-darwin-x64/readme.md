# `@wae/wae-darwin-x64`

macOS Intel host entry. Apple Silicon should use darwin-arm64, not this package.

## For application users

| Item                       | Details                                                                                                  |
|----------------------------|----------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-darwin-x64` @ `0.0.0`                                                                          |
| Platform id                | `darwin-x64`                                                                                             |
| Who needs it               | macOS Intel desktop WebView                                                                              |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-darwin-x64@0.0.0` |
| os/cpu                     | os: darwin · cpu: x64                                                                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                   |
| Runtime requirements       | macOS + WKWebView/similar shell (planned); no SDK install docs because binary not published.             |
| Not supported / incomplete | No .app / dylib.                                                                                         |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                       |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-darwin-x64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Build on Intel Mac; signing and notarization not documented yet (not needed yet).

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
