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

#[derive(Debug, Clone)]
pub struct MetaInfo {
    pub path: String,
}

/// {
/// "appName":"springCloud-test",
/// "contextPath":"/springcloud",
/// "path":"/springcloud/order/path/{id}/name",
/// "pathDesc":"",
/// "rpcType":"springCloud",
/// "serviceName":"org.apache.shenyu.examples.springcloud.controller.OrderController",
/// "methodName":"testRestFul",
/// "rule_name":"/springcloud/order/path/{id}/name",
/// "parameterTypes":"java.lang.String",
/// "enabled":true,
/// "pluginNames":[],
/// "registerMetaData":false,
/// "timeMillis":1724062308618,
/// "addPrefixed":false
/// }

#[derive(Debug, Clone)]
pub struct UriInfo {
    pub path: String,
    pub rule_name: String,
    pub service_name: Option<String>,
    pub method_name: Option<String>,
}

pub enum EventType {
    REGISTER,

    UPDATED,

    DELETED,

    IGNORED,

    OFFLINE
}


use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub(crate) shenyu: ShenYuConfig,
}

#[derive(Debug, Deserialize)]
pub struct ShenYuConfig {
    pub register: RegisterConfig,
    pub uri: UriConfig,
}

#[derive(Debug, Deserialize)]
pub struct RegisterConfig {
    pub register_type: String,
    pub servers: String,
    pub props: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UriConfig {
    pub app_name: String,
    pub host: String,
    pub port: u16,
    pub context_path: String,
    pub environment: String,
    pub rpc_type: String,
}