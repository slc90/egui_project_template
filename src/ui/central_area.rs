use std::fmt;

use egui::{Context, Ui};

pub fn show_central_area(_ctx: &Context, ui: &mut Ui) {
    ui.label("Central Panel Area");
}

/// 中央区有哪些功能
pub enum CentralAreaFunctions {
    /// 主页
    Home,

    /// 配置设置
    ConfigSetting,

    /// 关于
    About,
}

impl fmt::Display for CentralAreaFunctions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CentralAreaFunctions::Home => write!(f, "主页"),
            CentralAreaFunctions::ConfigSetting => write!(f, "设置"),
            CentralAreaFunctions::About => write!(f, "关于"),
        }
    }
}
