# Apache ShenYu-Client-Rust

The Apache ShenYu Rust Client SDK is a Rust library for interacting with the Apache ShenYu gateway. This SDK allows you to easily integrate your Rust applications with the ShenYu gateway, providing a seamless way to manage and route your API requests.

## Installation

To use the Apache ShenYu Rust Client SDK in your project, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
serde = "1.0.190"
serde_json = "1.0.80"
reqwest = "0.12.5"
axum = "0.5"
tokio = { version = "1", features = ["full"] }
```

## Usage

Below is an example of how to create an Axum service using the ShenYuRouter and integrate it with the ShenYu gateway.

### Example

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

This example demonstrates how to set up a basic Axum service with the ShenYuRouter and register it with the ShenYu gateway. The `health_handler` and `create_user_handler` are simple async functions that handle HTTP requests.

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for more details.
