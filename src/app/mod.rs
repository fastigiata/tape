use eframe::{egui, IconData, Theme};
use crate::app::notify::Notifier;

mod core;
mod prelude;
mod pages;
mod component;
mod notify;

pub fn run_tape_app() {
    let options = eframe::NativeOptions {
        app_id: Some("tape".to_string()),
        icon_data: IconData::try_from_png_bytes(include_bytes!("../assets/tape.png")).ok(),
        always_on_top: false,
        centered: true,
        resizable: false,
        decorated: false,
        transparent: true,
        follow_system_theme: false,
        default_theme: Theme::Light,
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    match eframe::run_native(
        "Tape",
        options,
        Box::new(|cc| Box::new(core::TapeApp::new(cc))),
    ) {
        Ok(_) => {}
        Err(err) => {
            // notify the error
            println!("Error: {}", err);
            Notifier::notify(&format!("Error: {}", err));
        }
    };
}