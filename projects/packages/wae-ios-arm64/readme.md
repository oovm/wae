# `@wae/wae-ios-arm64`

iOS arm64 mobile shell entry. Different from macOS darwin-arm64 desktop package.

## For application users

| Item                       | Details                                                                                                 |
|----------------------------|---------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-ios-arm64` @ `0.0.0`                                                                          |
| Platform id                | `ios-arm64`                                                                                             |
| Who needs it               | iOS WebView shell                                                                                       |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-ios-arm64@0.0.0` |
| os/cpu                     | os: darwin · cpu: arm64 (package manager field; device is still iOS)                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                  |
| Runtime requirements       | iOS; minimum version not published; requires Xcode.                                                     |
| Not supported / incomplete | No .ipa / xcframework.                                                                                  |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                      |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-ios-arm64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Maintain on macOS + Xcode only; TS shell today.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
