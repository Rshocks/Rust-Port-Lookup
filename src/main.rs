mod gui;
mod scanner;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Port Scanner",
        options,
        Box::new(|_cc| Box::new(gui::App::default())),
    )
}
