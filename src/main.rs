use rustynes::ui::RustyNesUi;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RustyNES",
        options,
        Box::new(|cc| Box::new(RustyNesUi::new(cc))),
    );
}

