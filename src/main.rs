pub mod config;
pub mod logger;
pub mod ui;
pub mod utils;

use tracing::info;

use crate::config::config::CONFIG;
use crate::logger::init_log;
use crate::ui::fonts::add_font;
use crate::ui::main_window::MainWindowState;
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    let config = CONFIG.lock().unwrap();
    let _file_logger_work_guards = init_log(config.logger_config.is_record_only_main_program_log);
    info!("程序启动");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_config.width, config.window_config.height])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        &config.window_config.title,
        native_options,
        Box::new(|cc| {
            add_font(&cc.egui_ctx);
            Ok(Box::new(MainWindowState::default()))
        }),
    )
}
