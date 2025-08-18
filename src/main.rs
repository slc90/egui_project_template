fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "bs_toolbox",
        native_options,
        Box::new(|_| Ok(Box::new(App))),
    )
}

struct App;

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
