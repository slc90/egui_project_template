pub mod background_manager;
pub mod config;
pub mod logger;
pub mod ui;
pub mod utils;

use tokio::sync::mpsc;
use tokio::task;
use tracing::info;

use crate::background_manager::manager::{RECEIVE_UI_MESSAGE, SEND_TO_UI};
use crate::config::config::CONFIG;
use crate::logger::init_log;
use crate::ui::fonts::add_font;
use crate::ui::main_window::{MainWindowState, RECEIVE_BACKGROUND_MESSAGE, SEND_TO_BACKGROUND};
use crate::utils::constants::{WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH};

fn main() -> eframe::Result {
    let config = CONFIG.lock().unwrap();
    let _file_logger_work_guards = init_log(config.logger_config.is_record_only_main_program_log);
    info!("程序启动");
    // 创建tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let _runtime_guard = runtime.enter();
    // 创建两对mpsc管道,Ui和后台双向通信
    let (send_to_ui, receive_background_message) = mpsc::channel::<i64>(100);
    let (send_to_background, receive_ui_message) = mpsc::channel::<i64>(100);
    SEND_TO_BACKGROUND.init(send_to_background);
    RECEIVE_BACKGROUND_MESSAGE.init(receive_background_message);
    SEND_TO_UI.init(send_to_ui);
    RECEIVE_UI_MESSAGE.init(receive_ui_message);
    // 启动后台任务
    task::spawn(async move { background_manager::manager::background_task_dispatcher().await });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_config.width, config.window_config.height])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT]),
        centered: true,
        ..Default::default()
    };
    let window_title = config.window_config.title.clone();
    // 不要一直持有锁，不然UI线程中别的地方再次获取全局配置时，就会死锁
    drop(config);
    let eframe_result = eframe::run_native(
        &window_title,
        native_options,
        Box::new(|cc| {
            add_font(&cc.egui_ctx);
            Ok(Box::new(MainWindowState::default()))
        }),
    );
    //把后台线程都关掉
    runtime.shutdown_background();
    //返回给操作系统的结果
    eframe_result
}
