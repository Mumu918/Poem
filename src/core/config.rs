use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

use crate::utils::file_path_utils::get_file_path;

fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub limit: usize,
    pub max_record_count: usize,
    pub interval: u64,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        let file_path = get_file_path("Config.toml");

        // 如果文件不存在，则创建并写入默认配置
        if !file_exists(&file_path) {
            let mut file = fs::File::create(file_path).expect("Unable to create file");
            let default_config = Config {
                limit: 20,
                max_record_count: 100,
                interval: 500,
                port: 9182,
            };
            let config_str =
                toml::to_string(&default_config).expect("Failed to serialize Config.toml");
            file.write_all(config_str.as_bytes())
                .expect("Unable to write to file");

            return default_config;
        }

        let config_str = fs::read_to_string(file_path).unwrap();
        let config: Config =
            toml::from_str(&config_str).expect("Failed to deserialize Config.toml");
        config
    }
}
