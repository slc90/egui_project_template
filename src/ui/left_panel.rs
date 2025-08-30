use egui::{Button, Context, Ui, Vec2};
use strum::IntoEnumIterator;

use crate::ui::{
    central_area::{CentralAreaFunctions, CentralAreaState},
    main_window::send_to_background,
};

pub fn show_left_panel(_ctx: &Context, ui: &mut Ui, central_area_state: &mut CentralAreaState) {
    ui.vertical_centered_justified(|ui| {
        for (i, function) in CentralAreaFunctions::iter().enumerate() {
            {
                if ui
                    .add(
                        Button::new(function.to_string())
                            .corner_radius(4)
                            .min_size(Vec2::new(0.0, 50.0)),
                    )
                    .clicked()
                {
                    central_area_state.current_function = function;
                    send_to_background(i as i64);
                }
            }
        }
    });
}
