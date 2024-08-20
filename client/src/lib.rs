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

//! Rust client sdk of Apache ShenYu.

pub mod client;
pub mod error;
pub mod model;

#[cfg(feature = "axum")]
mod axum_impl {
    use super::model::{ShenYuConfig, UriInfo};
    use crate::client::ShenyuClient;
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
    /// use client::ShenYuRouter;
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
    /// let app = ShenYuRouter::new("shenyu_client_app")
    ///     .nest("/api", ShenYuRouter::new("api"))
    ///     .route("/health", get(health_handler))
    ///     .route("/users", post(create_user_handler));
    ///
    /// ```
    ///
    pub struct ShenYuRouter<S> {
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

        pub fn route(mut self, path: &str, method_router: MethodRouter<S>) -> Self {
            self.inner = self.inner.route(path, method_router);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: None,
            });
            self
        }

        pub fn route_service<T>(mut self, path: &str, service: T) -> Self
        where
            T: Service<Request, Error=Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.route_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: None,
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
        pub fn nest_service<T>(mut self, path: &str, service: T) -> Self
        where
            T: Service<Request, Error=Infallible> + Clone + Send + 'static,
            T::Response: IntoResponse,
            T::Future: Send + 'static,
        {
            self.inner = self.inner.nest_service(path, service);
            self.uri_infos.push(UriInfo {
                path: path.to_string(),
                rule_name: path.to_string(),
                service_name: None,
                method_name: None,
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

    impl<S> Into<Router<S>> for ShenYuRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        fn into(self) -> Router<S> {
            self.inner
        }
    }


    impl ShenyuClient {

        pub async fn parse<S>(path: &str, router: ShenYuRouter<S>, port: u16) -> Result<Self, String>
        where
            S: Clone + Send + Sync + 'static,
        {
            let config = ShenYuConfig::from_yaml_file(path).unwrap();
            Self::from(config, router, port).await
        }

        pub async fn from<S>(config: ShenYuConfig, router: ShenYuRouter<S>, port: u16) -> Result<Self, String>
        where
            S: Clone + Send + Sync + 'static,
        {
            Self::new(config, router.app_name.as_str(), router.uri_infos, port).await
        }
    }
}

#[cfg(feature = "axum")]
pub use axum_impl::*;

#[cfg(feature = "actix-web")]
mod actix_web_impl {}

#[cfg(feature = "actix-web")]
pub use actix_web_impl::*;


#[cfg(test)]
#[cfg(feature = "axum")]
mod tests {
    use crate::client::ShenyuClient;
    use crate::model::ShenYuConfig;
    use crate::ShenYuRouter;
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
        let params = [("userName", hashmap.get("username").clone()), ("password", hashmap.get("password").clone())];

        // Fix the URL to include the scheme
        let res = client.get("http://127.0.0.1:9095/platform/login").query(&params).send().await.unwrap();
        let res_data: Value = res.json().await.unwrap();
        print!("res_data: {:?}", res_data);
        print!("res_data:token {:?}", res_data["data"]["token"]);
    }

    #[tokio::test]
    async fn build_client() {
        let app = ShenYuRouter::<()>::new("shenyu_client_app")
            .nest("/api", ShenYuRouter::new("api"))
            .route("/health", get(health_handler))
            .route("/users", post(create_user_handler));
        let config = ShenYuConfig::from_yaml_file("config.yml").unwrap();
        let res = ShenyuClient::from(config, app, 9527).await;
        assert!(&res.is_ok());
        let client = &mut res.unwrap();
        println!("client.token: {:?}", client.headers.get("X-Access-Token").unwrap_or(&"None".to_string()));

        let res = client.register_all_metadata(true).await;
        assert!(res.is_ok());
        let res = client.register_uri().await;
        assert!(res.is_ok());
    }

    #[test]
    fn it_works() {
        let binding = ShenYuRouter::<()>::new("shenyu_client_app");
        let app = binding
            .nest("/api", ShenYuRouter::new("api"))
            .route("/health", get(health_handler))
            .route("/users", post(create_user_handler));
        let uri_infos = app.uri_infos();
        assert_eq!(uri_infos.len(), 2);
        assert_eq!(uri_infos[0].path, "/health");
        assert_eq!(uri_infos[1].path, "/users");
    }
}
