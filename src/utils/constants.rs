/// 窗口最小宽度
pub const WINDOW_MIN_WIDTH: f32 = 1920.0;

/// 窗口最小高度
pub const WINDOW_MIN_HEIGHT: f32 = 1080.0;

/// 字体名称
pub const FONT_NAME: &str = "SmileySans-Oblique";

/// 存放日志的文件夹
pub const LOG_FOLDER: &str = "logs";

/// 默认配置的路径，这个里面会保留注释
pub const DEFAULT_CONFIG_PATH: &str = "config/default_config.toml";

/// 首先使用的配置路径,如果这不到就使用上面的默认配置
/// 在程序中修改配置中的属性后保存到的也是这个路径
/// 不要去修改上面的默认配置
pub const CONFIG_PATH: &str = "config/config.toml";
