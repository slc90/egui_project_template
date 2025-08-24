use egui::{CentralPanel, SidePanel, TopBottomPanel};
use tracing::info;

use crate::ui::{
    central_area::{CentralAreaState, show_central_area},
    left_panel::show_left_panel,
    menu_bar::show_menu_bar,
};

/// 主窗口需要管理的状态
#[derive(Default)]
pub struct MainWindowState {
    pub central_area_state: CentralAreaState,
}

impl eframe::App for MainWindowState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
