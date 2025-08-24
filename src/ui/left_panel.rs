use egui::{Context, Ui};

use crate::ui::central_area::CentralAreaFunctions;

pub fn show_left_panel(_ctx: &Context, ui: &mut Ui) {
    if ui.button(CentralAreaFunctions::Home.to_string()).clicked() {}
    if ui
        .button(CentralAreaFunctions::ConfigSetting.to_string())
        .clicked()
    {}
    if ui.button(CentralAreaFunctions::About.to_string()).clicked() {}
}
