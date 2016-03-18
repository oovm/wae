# communication

边界通信功能区。

| 子目录 | npm / crate | 规则 |
|--------|-------------|------|
| public-types | `@wae/types` | 仅类型，无 runtime；由 schema codegen |
| core | `@wae/core` | 跨 client/server 共享 runtime（非 UI、非 HTTP app） |
| protocol | `@wae/protocol` | 协议编解码 / envelope runtime |
| schema | crate `wae-types` | Rust IDL 真理源 → codegen |
| codecs | （可选） | 编码实现 |

依赖单向：`types → core|protocol → client|server`。禁止 `-js`/`-ts` 双胞胎目录。
