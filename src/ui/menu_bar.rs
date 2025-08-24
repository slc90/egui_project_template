use egui::{MenuBar, Ui};

pub fn show_menu_bar(_ctx: &egui::Context, ui: &mut Ui) {
    MenuBar::new().ui(ui, |ui| {
        //菜单栏可以嵌套
        ui.menu_button("Item1-Level1", |ui| {
            ui.menu_button("Item1-Level2", |_ui| {});
        });
        ui.menu_button("Item2-Level1", |_ui| {});
    });
}
