# @wae/core

跨 client/server 的**极窄**纯 TS 内核。

允许：Result/错误辅助、环境无关事件原语、序列化小工具。

禁止：server 应用、client runtime、UI、bridge、platform 探测、路由、WebSocket、Node/Deno/CF API。

若只剩类型，应并入 `@wae/types`，不要为保留目录而保留本包。
