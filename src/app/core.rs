use eframe::egui;

pub struct TapeApp {}

impl TapeApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for TapeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                ui.label("Hello world!");
            });
    }
}