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
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;

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
    let router_arc = Arc::new(Mutex::new(ShenYuRouter::new("shenyu_client_app")));
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();

    let client = {
        sleep(std::time::Duration::from_secs(10)).await;
        let binding = Arc::clone(&router_arc);
        let router_clone = binding.lock().unwrap();
        let res = ShenyuClient::from(
            config,
            router_clone.app_name(),
            router_clone.uri_infos(),
            4000,
        );
        let client = res.unwrap();
        client
    };

    let binding = Arc::clone(&router_arc);
    let server = HttpServer::new(move || {
        let mut router_clone = binding.lock().unwrap();
        let mut app = App::new().wrap(middleware::Logger::default());
        let router_clone = router_clone.deref_mut();
        shenyu_router!(
            router_clone,
            app,
            "/health" => get(health_handler)
            "/create_user" => post(create_user_handler)
            "/" => get(index)
        );

        app
    })
    .bind(("127.0.0.1", 4000))
    .expect("Can not bind to 4000")
    .run();

    client.register().await.expect("Failed to register");

    server.await.expect("Failed to start server");

    // Add shutdown hook
    tokio::select! {
        _ = signal::ctrl_c() => {
            client.offline_register().await;
        }
    }

    Ok(())
}
