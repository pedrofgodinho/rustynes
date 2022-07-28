use egui::Vec2;
use rustynes::ui::RustyNesUi;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2 {
            x: 1300.0,
            y: 800.0,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "RustyNES",
        options,
        Box::new(|cc| Box::new(RustyNesUi::new(cc))),
    );
}
