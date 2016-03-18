# examples — 产品形态验证矩阵

示例不是正式架构层。每个示例只表达一个主场景。

| 示例 | 主场景 |
|------|--------|
| frontend-counter | 纯 client |
| frontend-form | client + ui |
| frontend-ui / router / storage | client 专项 |
| desktop-webview | client + desktop platform |
| mobile-webview | client + mobile（可契约占位） |
| wasm-webview | **显式** wasm 能力（非默认 client 后端） |
| fullstack-ssr / auth / todo | server + client |
| backend-* | server / serverless adapter |

禁止：示例适配代码反向沉淀进正式包；正式包依赖 `@wae-example/*`。
