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

use crate::model::{EnvConfig, ShenYuConfig};
use serde_yaml;
use std::fs::File;
use std::io::Read;

impl ShenYuConfig {

    pub fn from_yaml_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: EnvConfig = serde_yaml::from_str(&contents)?;
        Ok(config.shenyu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_from_yaml_file() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let config_path = current_dir.join("config.yml");
        print!("config_path: {:?}", config_path);
        let config = ShenYuConfig::from_yaml_file(config_path.to_str().unwrap()).unwrap();
        assert_eq!(config.register.register_type, "http");
        assert_eq!(config.register.servers, "xx.xx.xx.x1");
        assert_eq!(config.register.props.len(), 2);
    }
}