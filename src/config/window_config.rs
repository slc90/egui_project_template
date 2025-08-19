use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowConfig {
    /// 标题
    pub title: String,

    /// 宽度
    pub width: f32,

    ///高度
    pub height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "BS Toolbox".to_string(),
            width: 1920.0,
            height: 1080.0,
        }
    }
}
