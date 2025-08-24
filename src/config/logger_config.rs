use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggerConfig {
    /// 是否只记录主程序的log,不记录三方库的log
    pub is_record_only_main_program_log: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            is_record_only_main_program_log: true,
        }
    }
}
