use egui::{Context, Ui};
use std::fmt;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
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

/// 主功能区需要管理的状态
pub struct CentralAreaState {
    /// 当前显示哪个功能
    pub current_function: CentralAreaFunctions,
}

impl Default for CentralAreaState {
    fn default() -> Self {
        // 默认显示主页
        CentralAreaState {
            current_function: CentralAreaFunctions::Home,
        }
    }
}

pub fn show_central_area(_ctx: &Context, ui: &mut Ui, central_area_state: &CentralAreaState) {
    match central_area_state.current_function {
        CentralAreaFunctions::Home => ui.label("主页"),
        CentralAreaFunctions::ConfigSetting => ui.label("设置"),
        CentralAreaFunctions::About => ui.label("关于"),
    };
}
