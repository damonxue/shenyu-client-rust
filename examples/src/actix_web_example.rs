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

use actix_web::rt::signal;
use actix_web::{middleware, web, App, HttpServer, Responder};
use shenyu_client_rust::actix_web_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{core::ShenyuClient, shenyu_router, IRouter};

async fn health_handler() -> impl Responder {
    "OK"
}

async fn create_user_handler() -> impl Responder {
    "User created"
}

async fn index() -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut router = ShenYuRouter::new("shenyu_client_app");
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();

    let server = HttpServer::new(move || {
            let app = App::new().wrap(middleware::Logger::default());
            shenyu_router!(
                router,
                app,
                "/health" => get(health_handler)
                "/create_user" => post(create_user_handler)
                "/" => get(index)
            );
            app
        })
        .bind(("127.0.0.1", 8080))?
        .run();
    let res = ShenyuClient::from(config, router.app_name(), router.uri_infos(), 9527).await;


    let client = &mut res.unwrap();
    client.register_all_metadata(true).await.expect("Failed to register metadata");
    client.register_uri().await.expect("Failed to register URI");
    client.register_discovery_config().await.expect("Failed to register discovery config");

    // Add shutdown hook
    tokio::select! {
        _ = server => {Ok(())},
        _ = signal::ctrl_c() => {
            let _ = client.offline_register().await;
            Ok(())
        }
    }
}