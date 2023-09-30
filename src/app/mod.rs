use eframe::{egui, IconData, Theme};

mod core;
mod helper;
mod misc;
mod pages;
mod prefab;

pub fn run_tape_app() {
    let options = eframe::NativeOptions {
        icon_data: IconData::try_from_png_bytes(include_bytes!("../assets/tape.png")).ok(),
        always_on_top: true,
        centered: true,
        resizable: false,
        decorated: false,
        transparent: true,
        follow_system_theme: false,
        default_theme: Theme::Light,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    match eframe::run_native(
        "Tape",
        options,
        Box::new(|cc| Box::new(core::TapeApp::new(cc))),
    ) {
        Ok(_) => {}
        Err(err) => {
            // TODO: notify the error
            println!("Error: {}", err);
        }
    };
}