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

#![cfg(feature = "axum")]
use axum::routing::post;
use axum::{routing::get, Router};
use shenyu_client_rust::axum_impl::ShenYuRouter;
use shenyu_client_rust::config::ShenYuConfig;
use shenyu_client_rust::{core::ShenyuClient, IRouter};

async fn health_handler() -> &'static str {
    "OK"
}

async fn create_user_handler() -> &'static str {
    "User created"
}

#[tokio::main]
async fn main() {
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
    let app = ShenYuRouter::<()>::new("shenyu_client_app")
        .nest("/api", ShenYuRouter::new("api"))
        .route("/health", "get", get(health_handler))
        .route("/users", "post", post(create_user_handler));
    let config = ShenYuConfig::from_yaml_file("examples/config.yml").unwrap();
    let client = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 3000).unwrap();

    let axum_app: Router = app.into();
    client.register().expect("TODO: panic message");
    ctrlc::set_handler(move || {
        client.offline_register();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Start Axum server
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        axum_app,
    )
    .await
    .unwrap();
}
