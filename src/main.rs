pub mod config;
pub mod logger;
pub mod utils;

use tracing::{debug, error, info};

use crate::config::config::CONFIG;
use crate::logger::init_log;
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    let config = CONFIG.lock().unwrap();
    let _work_guards = init_log(config.logger_config.is_record_only_main_program_log);
    debug!("debug");
    info!("info");
    error!("error");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_config.width, config.window_config.height])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        &config.window_config.title,
        native_options,
        Box::new(|_| Ok(Box::new(App))),
    )
}

struct App;

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
