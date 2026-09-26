# `@wae/wae-android-arm64`

Android arm64 mobile shell entry. Android SDK / NDK packaging flow not wired yet.

## For application users

| Item                       | Details                                                                                                     |
|----------------------------|-------------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-android-arm64` @ `0.0.0`                                                                          |
| Platform id                | `android-arm64`                                                                                             |
| Who needs it               | Android WebView shell                                                                                       |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-android-arm64@0.0.0` |
| os/cpu                     | cpu: arm64 (no Node os field; not a Node plugin)                                                            |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                      |
| Runtime requirements       | Android arm64-v8a; minimum API level not published.                                                         |
| Not supported / incomplete | No APK/AAB; cannot run directly with node.                                                                  |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                          |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-android-arm64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Requires Android Studio / SDK; TS shell only today.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
