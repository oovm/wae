# CORS 中间件

提供跨域资源共享 (Cross-Origin Resource Sharing) 配置。

## 示例

```ignore
use wae_https::middleware::CorsConfig;

let config = CorsConfig::new()
    .allow_origin("https://example.com")
    .allow_methods(["GET", "POST"])
    .allow_headers(["Content-Type", "Authorization"])
    .allow_credentials(true)
    .max_age(3600);

let cors_layer = config.into_layer();
```
