use eframe::{egui, IconData};

mod core;
mod pages;
mod prefabs;
mod misc;

pub fn run_tape_app() {
    let options = eframe::NativeOptions {
        icon_data: IconData::try_from_png_bytes(include_bytes!("../assets/tape.png")).ok(),
        always_on_top: false,
        centered: true,
        resizable: false,
        decorated: false,
        transparent: true,
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
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

#[cfg(test)]
mod unit_test {
    #[test]
    fn run() {}
}