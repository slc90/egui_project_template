use egui::{MenuBar, Ui};
use tracing::info;

use crate::config::{
    config::CONFIG,
    language_config::{Language, LanguageKey, translations},
};

pub fn show_menu_bar(_ctx: &egui::Context, ui: &mut Ui) {
    MenuBar::new().ui(ui, |ui| {
        //菜单栏可以嵌套
        ui.menu_button(translations(&LanguageKey::Settings), |ui| {
            ui.horizontal(|ui| {
                ui.menu_button(translations(&LanguageKey::Language), |ui: &mut Ui| {
                    if ui.button("中文").clicked() {
                        let mut config = CONFIG.lock().unwrap();
                        config.language_config.language = Language::Chinese;
                        config.save();
                        info!("语言切换成中文");
                    }
                    if ui.button("English").clicked() {
                        let mut config = CONFIG.lock().unwrap();
                        config.language_config.language = Language::English;
                        config.save();
                        info!("语言切换成英文");
                    }
                });
            })
        });
    });
}
