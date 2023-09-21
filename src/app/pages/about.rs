use eframe::egui;
use eframe::egui::{Align, Rect, Ui};
use crate::app::core::{AppRoute, TapeApp};
use crate::app::prefabs;

const TAPE_DESC_ZH: &'static str = "Tape 是一个用于记录鼠标和/或键盘操作的应用程序，可以生成记录文件用于稍后播放。";
const TAPE_DESC_EN: &'static str = "Tape is an application for recording mouse and/or keyboard actions, which can generate a recording file for later playback.";

pub fn about_renderer(app: &mut TapeApp, rect: Rect, ui: &mut Ui, frame: &mut eframe::Frame) {
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
            // ===== section: description
            ui.label(prefabs::text_zh("说明 / Description", 21.0));
            ui.label(prefabs::text_zh(TAPE_DESC_ZH, 16.0));
            ui.label(prefabs::text_en(TAPE_DESC_EN, 16.0));
            // TODO: divider line
        });

        ui.style().text_styles();

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
    });
}