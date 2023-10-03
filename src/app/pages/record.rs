use eframe::egui;
use eframe::egui::{Rect, Ui};
use crate::app::core::TapeApp;

pub fn record_renderer(_app: &mut TapeApp, rect: Rect, ui: &mut Ui, _frame: &mut eframe::Frame) {
    ui.allocate_ui_at_rect(
        {
            let mut v = rect;
            v.min += egui::vec2(16.0, 16.0);
            v.max -= egui::vec2(16.0, 16.0);
            v
        },
        |ui| {
            ui.label("Record");
        },
    );
}