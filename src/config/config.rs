use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Mutex};

use crate::config::window_config::WindowConfig;

/// 全局配置单例
pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let content = fs::read_to_string("config/config.toml").unwrap_or_default();
    let cfg: Config = toml::from_str(&content).unwrap_or_default();
    Mutex::new(cfg)
});

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    /// 窗口相关配置
    pub window_config: WindowConfig,
}
