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

//! Rust shenyu-client-rust sdk of Apache ShenYu.

use crate::model::UriInfo;

pub mod config;
pub mod core;
pub mod error;
pub mod macros;
pub mod model;

pub trait IRouter {
    fn app_name(&self) -> &str;

    fn uri_infos(&self) -> &Vec<UriInfo>;
}

#[cfg(feature = "axum")]
pub mod axum_impl {
    use super::model::UriInfo;
    use crate::IRouter;
    use axum::extract::Request;
    use axum::response::IntoResponse;
    use axum::routing::MethodRouter;
    use axum::Router;
    use std::convert::Infallible;
    use tower_service::Service;

    /// A router that can be used to register routes.
    ///
    /// This is a wrapper around [`Router`] that provides a more ergonomic API.
    /// ```rust
    ///
    /// use axum::routing::{get, post};///
    ///
    /// async fn health_handler() -> &'static str {
    ///     "OK"
    /// }
    ///
    /// async fn create_user_handler() -> &'static str {
    ///     "User created"
    /// }
    ///
    /// async fn not_found_handler() -> &'static str {
    ///     "Not found"
    /// }
    ///
    /// let app = ShenYuRouter::<()>::new("shenyu_client_app")
    ///     .nest("/api", ShenYuRouter::new("api"))
    ///     .route("/health", get(health_handler))
    ///     .route("/users", post(create_user_handler));
    ///
    /// ```
    ///
    #[derive(Debug, Clone)]
    pub struct ShenYuRouter<S = ()> {
        app_name: String,
        inner: Router<S>,
        uri_infos: Vec<UriInfo>,
    }

    impl<S> ShenYuRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        pub fn new(app_name: &str) -> Self {
            Self {
                app_name: app_name.to_string(),
                inner: Router::new(),
                uri_infos: Vec::new(),
            }
        }

        pub fn uri_info(mut self, uri_info: UriInfo) -> Self {
            self.uri_infos.push(uri_info);
            self
        }

        pub fn route(mut self, path: &str, method: &str, method_router: MethodRouter<S>) -> Self {
            self.inner = self.inner.route(path, method_router);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        pub fn route_service<T>(mut self, path: &str, method: &str, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.route_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        #[track_caller]
        pub fn nest(mut self, path: &str, route: ShenYuRouter<S>) -> Self {
            self.inner = self.inner.nest(path, route.inner);
            self.uri_infos.extend(route.uri_infos);
            self
        }

        #[track_caller]
        pub fn nest_service<T>(mut self, path: &str, method: &str, service: T) -> Self
        where
            T: Service<Request, Error = Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.nest_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: method.to_string(),
            });
            self
        }

        pub fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }

        #[track_caller]
        pub fn merge<R>(mut self, other: ShenYuRouter<R>) -> Self
        where
            R: Into<Router<S>>,
            S: Clone + Send + Sync + 'static,
            Router<S>: From<Router<R>>,
        {
            self.inner = self.inner.merge(other.inner);
            self.uri_infos.extend(other.uri_infos);
            self
        }
    }

    impl<S> From<ShenYuRouter<S>> for Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        fn from(val: ShenYuRouter<S>) -> Self {
            val.inner
        }
    }

    impl<S> IRouter for ShenYuRouter<S> {
        fn app_name(&self) -> &str {
            &self.app_name
        }

        fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }
    }
}

impl ShenyuClient {
    pub fn parse(path: &str, router: Box<dyn IRouter>, port: u16) -> Result<Self, String> {
        let config = ShenYuConfig::from_yaml_file(path).unwrap();
        Self::from(config, router.app_name(), router.uri_infos(), port)
    }

    pub fn from(
        config: ShenYuConfig,
        app_name: &str,
        uri_infos: &[UriInfo],
        port: u16,
    ) -> Result<Self, String> {
        Self::new(config, app_name, uri_infos, port)
    }
}

#[cfg(feature = "actix-web")]
pub mod actix_web_impl {
    use super::model::UriInfo;
    use crate::IRouter;

    /// A router that can be used to register routes.
    ///
    /// This is a wrapper around [`Router`] that provides a more ergonomic API.
    /// ```rust
    ///
    /// use actix_web::{web, get, post, App};
    /// use client::config::ShenYuConfig;    ///
    ///
    ///
    ///
    /// use client::core::ShenyuClient;
    ///
    /// async fn health_handler() -> &'static str {
    ///     "OK"
    /// }
    ///
    /// async fn create_user_handler() -> &'static str {
    ///     "User created"
    /// }
    ///
    /// async fn index() -> &'static str {
    ///     "Welcome!"
    /// }
    ///
    /// let router = ShenYuRouter::new("shenyu_client_app")
    ///     .route("/health", web::get(), health_handler)
    ///     .route("/users", web::post(), create_user_handler)
    ///     .service("/index.html", web::get(), index);
    /// let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
    /// let res = ShenyuClient::from(config, router.app_name(), router.uri_infos(), 9527).await;
    /// res.unwrap().init().await;
    /// let resouces = router.resources();
    /// let app = App::new().service(resouces);
    /// // then app is same as actix_web::App
    ///
    /// ```
    ///
    #[derive(Debug, Clone)]
    pub struct ShenYuRouter {
        app_name: String,
        uri_infos: Vec<UriInfo>,
    }

    impl ShenYuRouter {
        pub fn new(app_name: &str) -> Self {
            Self {
                app_name: app_name.to_string(),
                uri_infos: Vec::new(),
            }
        }

        pub fn route(&mut self, path: &str, method: &str) {
            self.uri_infos.push(UriInfo {
                path: path.to_string().clone(),
                rule_name: path.to_string().clone(),
                service_name: None,
                method_name: method.to_string(),
            });
        }
    }

    impl IRouter for ShenYuRouter {
        fn app_name(&self) -> &str {
            &self.app_name
        }

        fn uri_infos(&self) -> &Vec<UriInfo> {
            &self.uri_infos
        }
    }

    #[macro_export]
    macro_rules! shenyu_router {
        ($router:expr, $app:expr, $($path:expr => $method:ident($handler:expr))*) => {
            $(
                $router.route($path, stringify!($method));
                $app = $app.service(web::resource($path).route(web::$method().to($handler)));
            )*
        }
    }
}

use crate::config::ShenYuConfig;
use crate::core::ShenyuClient;

#[cfg(test)]
#[cfg(feature = "axum")]
mod tests_axum {
    use super::axum_impl::ShenYuRouter;
    use crate::config::ShenYuConfig;
    use crate::core::ShenyuClient;
    use crate::IRouter;
    use axum::routing::{get, post};
    use reqwest::Client;
    use serde_json::Value;
    use std::collections::HashMap;

    async fn health_handler() -> &'static str {
        "OK"
    }

    async fn create_user_handler() -> &'static str {
        "User created"
    }

    #[tokio::test]
    async fn test_login() {
        let client = Client::new();
        let mut hashmap = HashMap::new();
        hashmap.insert("username", "admin");
        hashmap.insert("password", "123456");
        let params = [
            ("userName", hashmap.get("username").clone()),
            ("password", hashmap.get("password").clone()),
        ];

        // Fix the URL to include the scheme
        let res = client
            .get("http://127.0.0.1:9095/platform/login")
            .query(&params)
            .send()
            .await
            .unwrap();
        let res_data: Value = res.json().await.unwrap();
        print!("res_data: {:?}", res_data);
        print!("res_data:token {:?}", res_data["data"]["token"]);
    }

    #[tokio::test]
    async fn build_client() {
        let app = ShenYuRouter::<()>::new("shenyu_client_app")
            .nest("/api",ShenYuRouter::new("api"))
            .route("/health", "get", get(health_handler))
            .route("/users", "post", post(create_user_handler));
        let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
        let res = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 9527);
        assert!(&res.is_ok());
        let client = &mut res.unwrap();
        println!(
            "shenyu-client-rust.token: {:?}",
            client
                .headers
                .get("X-Access-Token")
                .map(|r| r.clone())
                .unwrap_or("None".to_string())
        );

        let res = client.register_all_metadata(true).await;
        assert!(res.is_ok());
        let res = client.register_uri().await;
        assert!(res.is_ok());
        let res = client.register_discovery_config().await;
        assert!(res.is_ok());
        client.offline_register().await;
    }

    #[test]
    fn it_works() {
        let binding = ShenYuRouter::<()>::new("shenyu_client_app");
        let app = binding
            .nest("/api", ShenYuRouter::new("api"))
            .route("/health", "get", get(health_handler))
            .route("/users", "post", post(create_user_handler));
        let uri_infos = app.uri_infos();
        assert_eq!(uri_infos.len(), 2);
        assert_eq!(uri_infos[0].path, "/health");
        assert_eq!(uri_infos[1].path, "/users");
    }
}

#[cfg(test)]
#[cfg(feature = "actix-web")]
mod tests_actix_web {
    use super::actix_web_impl::ShenYuRouter;
    use crate::config::ShenYuConfig;
    use crate::core::ShenyuClient;
    use crate::IRouter;

    #[tokio::test]
    async fn build_client() {
        let app = ShenYuRouter::new("shenyu_client_app");
        let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
        let res = ShenyuClient::from(config, app.app_name(), app.uri_infos(), 9527);
        assert!(&res.is_ok());
        let client = &mut res.unwrap();
        println!(
            "shenyu-client-rust.token: {:?}",
            client
                .headers
                .get("X-Access-Token")
                .map(|r| r.clone())
                .unwrap_or("None".to_string())
        );

        let res = client.register_all_metadata(true).await;
        assert!(res.is_ok());
        let res = client.register_uri().await;
        assert!(res.is_ok());
        let res = client.register_discovery_config().await;
        assert!(res.is_ok());
        client.offline_register().await;
    }
}
