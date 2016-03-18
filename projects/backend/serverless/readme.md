# @wae/serverless

统一适配层：把 Node / Deno / Cloudflare 等入口与 binding 转成 `@wae/server` 可理解的标准环境。

不是「某个云厂商的 serverless 产品」；具体平台包（`@wae/server-cloudflare` 等）依赖本包，而不是各自发明一套 server 语义。
