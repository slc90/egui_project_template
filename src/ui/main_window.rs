use egui::{CentralPanel, SidePanel, TopBottomPanel};
use singlyton::SingletonUninit;
use tokio::sync::mpsc::{Receiver, Sender, error::TryRecvError};
use tracing::{debug, info};

use crate::ui::{
    central_area::{CentralAreaState, show_central_area},
    left_panel::show_left_panel,
    menu_bar::show_menu_bar,
};

/// UI线程的sender和receiver
pub static SEND_TO_BACKGROUND: SingletonUninit<Sender<i64>> = SingletonUninit::uninit();
pub static RECEIVE_BACKGROUND_MESSAGE: SingletonUninit<Receiver<i64>> = SingletonUninit::uninit();

/// 因为egui中不能直接使用异步,所以封装在一个tokio::spawn中
/// 这个函数只能在Ui线程中使用
/// # Arguments
///
/// - `command` (`i64`) - Ui向后台发送的命令,实际应该用枚举,这里用整数只是为了验证通路
///
/// # Returns
///
/// - `()` - Describe the return value.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = send_to_background();
/// ```
pub(super) fn send_to_background(command: i64) -> () {
    tokio::spawn(async move {
        let _ = SEND_TO_BACKGROUND.get().send(command as i64).await;
    });
}

/// 主窗口需要管理的状态
#[derive(Default)]
pub struct MainWindowState {
    pub central_area_state: CentralAreaState,
}

impl eframe::App for MainWindowState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //每次重绘Ui前接收一下后台线程传过来的消息
        match RECEIVE_BACKGROUND_MESSAGE.get_mut().try_recv() {
            Ok(result) => {
                debug!("background result:{}", result);
                //根据这个结果对Ui的state作响应修改
            }
            Err(e) => match e {
                TryRecvError::Empty => {
                    //这个错误不需要处理
                }
                TryRecvError::Disconnected => panic!("未创建和后台线程的mpsc"),
            },
        }

        //顶部是菜单栏
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                show_menu_bar(ctx, ui);
            });
        //左侧用于切换主功能区中的内容
        SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                show_left_panel(ctx, ui, &mut self.central_area_state);
            });
        //中间放主功能区
        CentralPanel::default().show(ctx, |ui| {
            show_central_area(ctx, ui, &self.central_area_state);
        });
    }

    fn on_exit(&mut self) {
        info!("程序关闭");
    }
}
