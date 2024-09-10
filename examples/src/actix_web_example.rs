// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![cfg(feature = "actix-web")]
use actix_web::{web, App, HttpServer};
use actix_web::rt::signal;
use shenyu_client_rust::actix_web_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{IRouter, core::ShenyuClient};

async fn health_handler() -> &'static str {
    "OK"
}

async fn create_user_handler() -> &'static str {
    "User created"
}

async fn index() -> &'static str {
    "Welcome!"
}

#[tokio::main]
async fn main() {
    let app = ShenYuRouter::new("shenyu_client_app")
        .route("/health", web::get(), health_handler)
        .route("/users", web::post(), create_user_handler)
        .service("/index.html", web::get(), index);
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
    let res = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 9527).await;
    let resouces = app.resources();
    let actix_app = App::new().service(resouces);

    let client = &mut res.unwrap();
    client.register_all_metadata(true).await.expect("Failed to register metadata");
    client.register_uri().await.expect("Failed to register URI");
    client.register_discovery_config().await.expect("Failed to register discovery config");

    // Start Actix-web server
    let server = HttpServer::new(move || actix_app)
        .bind("0.0.0.0:4000")
        .unwrap()
        .run();

    // Add shutdown hook
    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            client.offline_register().await;
        }
    }
}