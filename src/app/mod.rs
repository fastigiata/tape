use eframe::{egui};

mod core;
mod pages;
mod components;

pub fn run_tape_app() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,  // false
        resizable: false,
        centered: true,
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
        ..Default::default()
    };

    match eframe::run_native(
        "Tape",
        options,
        Box::new(|_| Box::new(core::TapeApp::new())),
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