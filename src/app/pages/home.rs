use eframe::egui::{Rect, Ui, UserAttentionType};
use crate::app::core::{AppRoute, AppState, TapeApp};

pub fn home_renderer(app: &mut TapeApp, rect: Rect, ui: &mut Ui, frame: &mut eframe::Frame) {
    ui.allocate_ui_at_rect(rect, |ui| {
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
            app.set_app_route(AppRoute::History);
        }

        if ui.button("Go to About").clicked() {
            app.set_app_route(AppRoute::About);
        }

        ui.separator();

        if ui.button("Set State = Idle").clicked() {
            app.set_app_state(AppState::Idle);
        }

        if ui.button("Set State = Record").clicked() {
            app.set_app_state(AppState::Record);
        }

        if ui.button("Set State = Act").clicked() {
            app.set_app_state(AppState::Act);
        }

        ui.separator();

        if ui.button("Set Attention = Critical").clicked() {
            frame.request_user_attention(UserAttentionType::Critical);
        }

        if ui.button("Set Attention = Informational").clicked() {
            frame.request_user_attention(UserAttentionType::Informational);
        }

        if ui.button("Set Attention = Reset").clicked() {
            frame.request_user_attention(UserAttentionType::Reset);
        }
    });
}