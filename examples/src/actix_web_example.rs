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
use actix_web::{middleware, App, HttpServer, Responder};
use shenyu_client_rust::actix_web_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{core::ShenyuClient, register_once, shenyu_router, IRouter};

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
    std::thread::spawn(|| {
        // ctrl+c after 10 seconds, just for CI
        std::thread::sleep(std::time::Duration::from_secs(10));
        let pid = std::process::id() as _;
        unsafe {
            #[cfg(unix)]
            libc::kill(pid, libc::SIGINT);
            #[cfg(windows)]
            windows_sys::Win32::System::Console::GenerateConsoleCtrlEvent(
                windows_sys::Win32::System::Console::CTRL_C_EVENT,
                pid,
            );
        };
    });
    HttpServer::new(move || {
        let mut router = ShenYuRouter::new("shenyu_client_app");
        let mut app = App::new().wrap(middleware::Logger::default());
        let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
        shenyu_router!(
            router,
            app,
            "/health" => get(health_handler)
            "/create_user" => post(create_user_handler)
            "/" => get(index)
        );
        register_once!(config, router, 4000);

        app
    })
    .bind(("0.0.0.0", 4000))
    .expect("Can not bind to 4000")
    .run()
    .await
}
