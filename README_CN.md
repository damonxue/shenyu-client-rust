# Apache ShenYu-Client-Rust ShenYu-rust客户端

Apache ShenYu Rust 客户端 SDK 是一个用于与 Apache ShenYu 网关交互的 Rust 库。此 SDK 允许您轻松地将 Rust 应用程序与 ShenYu 网关集成，提供一种无缝的方式来管理和路由 API 请求。

## 安装

要在项目中使用 Apache ShenYu Rust 客户端 SDK，请在 `Cargo.toml` 文件中添加以下依赖项：

```toml
[dependencies]
serde = "1.0.190"
serde_json = "1.0.80"
reqwest = "0.12.5"
axum = "0.5"
tokio = { version = "1", features = ["full"] }
```

## 使用

下面是一个如何使用 ShenYuRouter 创建 Axum 服务并将其与 ShenYu 网关集成的示例。

### 示例

```rust
use axum::{routing::get, Router};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tokio::runtime::Runtime;

async fn health_handler() -> &'static str {
    "OK"
}

async fn create_user_handler() -> &'static str {
    "User created"
}

#[tokio::main]
async fn main() {
    let app = ShenYuRouter::<()>::new("shenyu_client_app")
        .nest("/api", ShenYuRouter::new("api"))
        .route("/health", get(health_handler))
        .route("/users", axum::routing::post(create_user_handler))
        .build();

    let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
    let res = ShenyuClient::from(config, app, 9527).await;
    assert!(res.is_ok());
    let client = &mut res.unwrap();
    println!("client.token: {:?}", client.headers.get("X-Access-Token").unwrap_or(&"None".to_string()));

    let res = client.register_all_metadata(true).await;
    assert!(res.is_ok());
    let res = client.register_uri().await;
    assert!(res.is_ok());
}
```

此示例演示了如何使用 ShenYuRouter 设置基本的 Axum 服务并将其注册到 ShenYu 网关。`health_handler` 和 `create_user_handler` 是处理 HTTP 请求的简单异步函数。

## 许可证

此项目根据 Apache 许可证 2.0 版获得许可。有关更多详细信息，请参阅 [LICENSE](LICENSE) 文件。
