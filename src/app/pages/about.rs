use eframe::egui::{Rect, Ui};
use crate::app::core::{AppRoute, TapeApp};
use crate::app::prefabs;

pub fn about_renderer(app: &mut TapeApp, rect: Rect, ui: &mut Ui, frame: &mut eframe::Frame) {
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.label(prefabs::text_en("Powered by egui", 16.0));
        ui.label(prefabs::text_zh("Powered by egui", 16.0));
        ui.label("This is just the contents of the window.");

        if ui.button("Go to Home").clicked() {
            app.set_app_route(AppRoute::Home);
        }

        if ui.button("Go to Record").clicked() {
            app.set_app_route(AppRoute::Record);
        }

        if ui.button("Go to Act").clicked() {
            app.set_app_route(AppRoute::Act);
        }

        if ui.button("Go to List").clicked() {
            app.set_app_route(AppRoute::List);
        }

        if ui.button("Go to About").clicked() {
            app.set_app_route(AppRoute::About);
        }
    });
}