pub mod config;
pub mod logger;
pub mod utils;

use crate::config::config::CONFIG;
use crate::logger::subcriber::test_log;
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    test_log();
    let config = CONFIG.lock().unwrap();
    let title = &config.window_config.title;
    let window_width = config.window_config.width;
    let window_height = config.window_config.height;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, window_height])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(&title, native_options, Box::new(|_| Ok(Box::new(App))))
}

struct App;

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
