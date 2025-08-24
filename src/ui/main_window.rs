use egui::{CentralPanel, SidePanel, TopBottomPanel};
use tracing::info;

use crate::ui::{
    central_area::show_central_area, left_panel::show_left_panel, menu_bar::show_menu_bar,
};

pub struct MainWindow;

impl eframe::App for MainWindow {
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
                show_left_panel(ctx, ui);
            });
        //中间放主功能区
        CentralPanel::default().show(ctx, |ui| {
            show_central_area(ctx, ui);
        });
    }

    fn on_exit(&mut self) {
        info!("程序关闭");
    }
}
