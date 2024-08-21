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

use crate::error::ShenYuError;
use crate::model::{EventType, ShenYuConfig, UriInfo};
use local_ip_address::local_ip;
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Error;

mod config;

pub const REGISTER_META_DATA_SUFFIX: &str = "/shenyu-client/register-metadata";
pub const REGISTER_URI_SUFFIX: &str = "/shenyu-client/register-uri";
pub const PLATFORM_LOGIN_SUFFIX: &str = "/platform/login";

#[derive(Debug)]
#[warn(dead_code)]
pub struct ShenyuClient {
    pub(crate) headers: HashMap<String, String>,
    app_name: String,
    env: ShenYuConfig,
    port: u16,
    gateway_base_urls: Vec<String>,
    register_meta_data_path_list: Vec<String>,
    register_uri_list: Vec<String>,
    register_token_servers: Vec<String>,
    uri_infos: Box<Vec<UriInfo>>,
}

impl ShenyuClient {
    pub async fn new(config: ShenYuConfig, app_name: &str, uri_infos: &Vec<UriInfo>, port: u16) -> Result<Self, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json;charset=UTF-8".to_string());

        let mut client = ShenyuClient {
            headers: HashMap::new(),
            app_name: app_name.to_string(),
            env: config,
            port,
            gateway_base_urls: vec![],
            register_meta_data_path_list: vec![],
            register_uri_list: vec![],
            register_token_servers: vec![],
            uri_infos: Box::new(uri_infos.clone()),
        };

        client.set_up_gateway_service_url()?;

        if let Ok(token) = client.get_register_token().await {
            headers.insert("X-Access-Token".to_string(), token.to_string());
            client.headers = headers;
            Ok(client)
        } else {
            Err("Can't get register token".to_string())
        }
    }
}

impl ShenyuClient {
    fn set_up_gateway_service_url(&mut self) -> Result<(), String> {
        self.gateway_base_urls = self.env.register.servers.split(',').map(|s| s.to_string()).collect();

        self.register_meta_data_path_list = self.gateway_base_urls.iter().map(|url| format!("{}{}", url, REGISTER_META_DATA_SUFFIX)).collect();
        self.register_uri_list = self.gateway_base_urls.iter().map(|url| format!("{}{}", url, REGISTER_URI_SUFFIX)).collect();
        self.register_token_servers = self.gateway_base_urls.iter().map(|url| format!("{}{}", url, PLATFORM_LOGIN_SUFFIX)).collect();

        Ok(())
    }

    async fn request(&self, url: &str, json_data: &Value) -> Result<bool, Error> {
        let client = Client::new();
        let mut builder = client.post(url).json(json_data);
        // 遍历header， 添加到builder中
        for (key, value) in &self.headers {
            builder = builder.header(key, value);
        }
        let res = builder.send().await.unwrap();
        let status_code = res.status();
        let msg = res.text().await.unwrap();

        if msg == "success" {
            Ok(true)
        } else {
            println!("Request ({}) failed, status code: {}, msg: {}", url, status_code, msg);
            Ok(false)
        }
    }

    async fn get_register_token(&mut self) -> Result<String, Error> {
        let client = Client::new();
        let hashmap = &self.env.register.props;
        let params = [("userName", hashmap.get("username").clone()), ("password", hashmap.get("password").clone())];

        let result = Err(ShenYuError::new(500, "Can't get register token".to_string()).into());
        for url in &self.register_token_servers {
            let res: Response = client.get(url).query(&params).send().await.unwrap();
            let res_data: Value = res.json().await.unwrap();
            match res_data.get("data")
                .and_then(|data| data.get("token"))
                .and_then(|token| token.as_str()) {
                Some(token) => return Ok(token.to_string()),
                None => continue,
            }
        }
        result
    }

    pub async fn register_uri(&self) -> Result<bool, Error> {
        let app_name = &self.app_name.clone();
        let rpc_type = &self.env.uri.rpc_type.clone();
        let context_path = &self.env.uri.context_path.clone();
        let host = match local_ip() {
            Ok(std::net::IpAddr::V4(ipv4)) => ipv4,
            Ok(std::net::IpAddr::V6(ipv6)) => ipv6.to_ipv4().unwrap(),
            _ => todo!("Handle error")
        };
        let port = &self.port;

        let json_data = serde_json::json!({
            "appName": app_name,
            "contextPath": context_path,
            "protocol": rpc_type,
            "host": host.to_string(),
            "port": port,
            "eventType": EventType::REGISTER.to_string(),
        });

        for url in &self.register_uri_list {
            if self.request(url, &json_data).await? {
                println!("[SUCCESS], register uri success, register data: {:?}", json_data);
                return Ok(true);
            }
        }

        println!("[ERROR], register uri failed, app_name: {}, host: {}, port: {}", app_name, host, port);
        Ok(false)
    }

    pub async fn register_all_metadata(&self, enabled: bool) -> Result<bool, Error> {
        for x in self.uri_infos.iter() {
            match self.register_metadata(false, Some(&x.path), Some(&x.rule_name), enabled).await {
                Ok(true) => continue,
                Ok(false) => return Ok(false),
                Err(e) => return Err(e),
            }
        }
        Ok(true)
    }

    async fn register_metadata(&self, register_all: bool, path: Option<&str>, rule_name: Option<&str>, enabled: bool) -> Result<bool, Error> {
        let context_path = &self.env.uri.context_path.clone();
        let app_name = &self.app_name.clone();
        let rpc_type = &self.env.uri.rpc_type.clone();
        let path = if register_all {
            format!("{}**", context_path)
        } else {
            path.unwrap_or("").to_string()
        };

        let rule_name = rule_name.unwrap_or(&path).to_string();
        let json_data = serde_json::json!({
            "appName": app_name.clone(),
            "contextPath": context_path.clone(),
            "path": context_path.clone() + &*path.clone(),
            "pathDesc": "",
            "rpcType": rpc_type,
            "rule_name": rule_name,
            "enabled": enabled,
            "registerMetaData": "",
            "pluginNames": []
        });

        for url in &self.register_meta_data_path_list {
            if self.request(url, &json_data).await? {
                println!("[SUCCESS], register metadata success, register data: {:?}", json_data);
                return Ok(true);
            }
        }

        println!("[ERROR], register metadata failed, app_name: {}, path: {}, contextPath: {}", app_name, path, context_path);
        Ok(false)
    }
}