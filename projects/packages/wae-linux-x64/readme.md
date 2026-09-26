# `@wae/wae-linux-x64`

Linux x64 host entry. Distro differences (glibc/musl, WebKitGTK) not fixed in 0.0.0.

## For application users

| Item                       | Details                                                                                                 |
|----------------------------|---------------------------------------------------------------------------------------------------------|
| npm package                | `@wae/wae-linux-x64` @ `0.0.0`                                                                          |
| Platform id                | `linux-x64`                                                                                             |
| Who needs it               | Linux x64 desktop WebView                                                                               |
| Install                    | Prefer `@wae/wae`; pulled via `optionalDependencies`; or explicitly `pnpm add @wae/wae-linux-x64@0.0.0` |
| os/cpu                     | os: linux · cpu: x64                                                                                    |
| Artifacts                  | `dist/index.js` + type declarations (TypeScript shell)                                                  |
| Runtime requirements       | Linux x64; minimum glibc / WebKit versions to be declared later.                                        |
| Not supported / incomplete | No ELF binary; no AppImage/Flatpak docs.                                                                |
| Host relationship          | Desktop/mobile: via Rust `host/bridge` IPC; web: no host; wasm: explicit Wasm path                      |

Verify loading (shell layer):

```ts
import platform from "@wae/wae-linux-x64";
console.log(platform.id);
```

`start` / `build` / `run` are no-op in 0.0.0; success only means API is callable, not that a window opened.

## For maintainers

Build on target distro; CI matrix does not cover all distros.

Do not mix application install steps with native cross-compilation in the same "quick start" section.

## Related

- Overview: [platform area](../../readme.md)
- Host: [wae-bridge](../../crates/wae-bridge/readme.md)
