use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    sync::Mutex,
};

use crate::{
    config::{
        language_config::LanguageConfig, logger_config::LoggerConfig, window_config::WindowConfig,
    },
    utils::constants::{CONFIG_PATH, DEFAULT_CONFIG_PATH},
};

/// 全局配置单例
pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    //先尝试读取实际使用的配置文件
    let result = fs::read_to_string(CONFIG_PATH);
    match result {
        Ok(content) => {
            let cfg: Config = toml::from_str(&content).unwrap_or_default();
            Mutex::new(cfg)
        }
        Err(_) => {
            //出错时就读取默认配置文件
            let result = fs::read_to_string(DEFAULT_CONFIG_PATH);
            match result {
                Ok(content) => {
                    let cfg: Config = toml::from_str(&content).unwrap_or_default();
                    Mutex::new(cfg)
                }
                //还出错时就使用默认配置
                Err(_) => Mutex::new(Config::default()),
            }
        }
    }
});

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    /// 窗口相关配置
    pub window_config: WindowConfig,

    /// 日志相关配置
    pub logger_config: LoggerConfig,

    /// 选择的语言
    pub language_config: LanguageConfig,
}

impl Config {
    /// 配置保存到文件
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - Describe this parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = save();
    /// ```
    pub fn save(&self) {
        let toml_str = toml::to_string_pretty(&self).unwrap();
        let mut file = File::create(CONFIG_PATH).unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();
    }
}
